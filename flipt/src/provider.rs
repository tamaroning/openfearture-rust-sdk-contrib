use std::collections::HashMap;

use async_trait::async_trait;
use flipt::{
    evaluation::models::EvaluationRequest, AuthenticationStrategy as FpAuthenticationStrategy,
};
use open_feature::{
    provider::{FeatureProvider, ProviderMetadata, ResolutionDetails},
    EvaluationContext, EvaluationError, EvaluationErrorCode, EvaluationResult, StructValue,
};
use serde_json::Value as JsonValue;
use url::Url;

use crate::utils::{reason_as_str, translate_context, translate_error, translate_value};

// reexports
pub use flipt::{ClientTokenAuthentication, JWTAuthentication, NoneAuthentication};

const DEFAULT_NAMESPACE: &str = "default";
const METADATA: &str = "flipt";

pub struct Config<A>
where
    A: FpAuthenticationStrategy,
{
    endpoint: String,
    auth_strategy: A,
    timeout: u64,
}

impl<A> Config<A>
where
    A: FpAuthenticationStrategy,
{
    pub fn new(endpoint: String, authentication_strategy: A, timeout: u64) -> Self {
        Self {
            endpoint,
            auth_strategy: authentication_strategy.into(),
            timeout,
        }
    }
}

pub struct FliptProvider {
    metadata: ProviderMetadata,
    client: flipt::api::FliptClient,
}

impl FliptProvider {
    pub fn new<A: FpAuthenticationStrategy>(config: Config<A>) -> Result<Self, String> {
        let url = match Url::parse(&config.endpoint) {
            Ok(url) => url,
            Err(e) => return Err(e.to_string()),
        };

        let flipt_config = flipt::Config::new(url, config.auth_strategy, config.timeout);
        let client = match flipt::api::FliptClient::new(flipt_config) {
            Ok(fpconfig) => fpconfig,
            Err(e) => return Err(e.to_string()),
        };

        Ok(Self {
            metadata: ProviderMetadata::new(METADATA),
            client,
        })
    }
}

impl Default for FliptProvider {
    fn default() -> Self {
        FliptProvider {
            metadata: ProviderMetadata::new(METADATA),
            client: flipt::api::FliptClient::default(),
        }
    }
}

fn error<T>(msg: &str) -> EvaluationResult<T> {
    Err(EvaluationError::builder()
        .code(EvaluationErrorCode::ProviderNotReady)
        .message(msg)
        .build())
}

#[async_trait]
impl FeatureProvider for FliptProvider {
    fn metadata(&self) -> &ProviderMetadata {
        &self.metadata
    }

    async fn resolve_bool_value(
        &self,
        flag_key: &str,
        ctx: &EvaluationContext,
    ) -> EvaluationResult<ResolutionDetails<bool>> {
        let res = match self
            .client
            .evaluation
            .boolean(&EvaluationRequest {
                namespace_key: ctx
                    .targeting_key
                    .clone()
                    .unwrap_or(DEFAULT_NAMESPACE.into()),
                flag_key: flag_key.into(),
                entity_id: "entity".into(),
                context: translate_context(ctx),
                reference: None,
            })
            .await
        {
            Ok(r) => r,
            Err(e) => return Err(translate_error(e)),
        };

        EvaluationResult::Ok(ResolutionDetails::new(res.enabled))
    }

    async fn resolve_int_value(
        &self,
        _flag_key: &str,
        _evaluation_context: &EvaluationContext,
    ) -> EvaluationResult<ResolutionDetails<i64>> {
        error("flipt does not support int values")
    }

    async fn resolve_float_value(
        &self,
        _flag_key: &str,
        _evaluation_context: &EvaluationContext,
    ) -> EvaluationResult<ResolutionDetails<f64>> {
        error("flipt does not support float values")
    }

    async fn resolve_string_value(
        &self,
        _flag_key: &str,
        _evaluation_context: &EvaluationContext,
    ) -> EvaluationResult<ResolutionDetails<String>> {
        error("flipt does not support string values")
    }

    async fn resolve_struct_value(
        &self,
        flag_key: &str,
        ctx: &EvaluationContext,
    ) -> Result<ResolutionDetails<StructValue>, EvaluationError> {
        let res = match self
            .client
            .evaluation
            .variant(&EvaluationRequest {
                namespace_key: ctx
                    .targeting_key
                    .clone()
                    .unwrap_or(DEFAULT_NAMESPACE.into()),
                flag_key: flag_key.into(),
                entity_id: "entity".into(),
                context: translate_context(ctx),
                reference: None,
            })
            .await
        {
            Ok(r) => r,
            Err(e) => return Err(translate_error(e)),
        };

        dbg!(&res);

        if !res.r#match {
            return Err(EvaluationError::builder()
                .code(EvaluationErrorCode::General(
                    reason_as_str(&res.reason).to_owned(),
                ))
                .build());
        }

        let v: JsonValue = match serde_json::from_str(&res.variant_attachment) {
            Ok(v) => v,
            Err(e) => {
                return Err(EvaluationError::builder()
                    .code(EvaluationErrorCode::General(
                        "invalid variant attachment".into(),
                    ))
                    .message(e.to_string())
                    .build())
            }
        };

        let object = match v.as_object() {
            Some(o) => o,
            None => {
                return Err(EvaluationError::builder()
                    .code(EvaluationErrorCode::General(
                        "invalid variant attachment".into(),
                    ))
                    .message(format!(
                        "variant attachment must be an object, but found `{}`",
                        v.to_string()
                    ))
                    .build())
            }
        };

        let mut fields = HashMap::new();
        for (k, v) in object {
            match translate_value(v.clone()) {
                Ok(v) => fields.insert(k.clone(), v),
                Err(e) => {
                    return Err(EvaluationError::builder()
                        .code(EvaluationErrorCode::General(
                            "invalid variant attachment".into(),
                        ))
                        .message(e)
                        .build())
                }
            };
        }
        Ok(ResolutionDetails::new(StructValue { fields }))
    }
}
