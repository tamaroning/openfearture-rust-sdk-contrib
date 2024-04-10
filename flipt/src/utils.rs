use flipt::{error::UpstreamError, evaluation::models::EvaluationReason};
use open_feature::{
    EvaluationContext, EvaluationError, EvaluationErrorCode, StructValue, Value as OpenFeatureValue,
};
use serde_json::Value as SerdeValue;
use std::collections::HashMap;

pub(crate) fn translate_error(e: UpstreamError) -> EvaluationError {
    EvaluationError {
        code: EvaluationErrorCode::General(e.code.to_string()),
        message: Some(e.message),
    }
}

pub(crate) fn translate_context(ctx: &EvaluationContext) -> HashMap<String, String> {
    let mut map: HashMap<String, String> = HashMap::new();
    for (k, v) in ctx.custom_fields.iter() {
        if let Some(v) = v.as_str() {
            map.insert(k.clone(), v.to_owned());
        };
    }
    map
}

pub(crate) fn translate_value(v: SerdeValue) -> Result<OpenFeatureValue, String> {
    match v {
        SerdeValue::Bool(b) => Ok(OpenFeatureValue::Bool(b)),
        SerdeValue::Number(n) => {
            if let Some(i) = n.as_i64() {
                Ok(OpenFeatureValue::Int(i))
            } else {
                Ok(OpenFeatureValue::Float(n.as_f64().unwrap()))
            }
        }
        SerdeValue::String(s) => Ok(OpenFeatureValue::String(s)),
        SerdeValue::Object(m) => {
            let mut fields = HashMap::new();
            for (k, v) in m.into_iter() {
                fields.insert(k.clone(), translate_value(v)?);
            }
            Ok(OpenFeatureValue::Struct(StructValue { fields }))
        }
        SerdeValue::Array(arr) => {
            let values = arr
                .into_iter()
                .map(translate_value)
                .collect::<Result<Vec<_>, _>>()?;
            Ok(OpenFeatureValue::Array(values))
        }
        SerdeValue::Null => Err("null value is not supported".to_string()),
    }
}

pub(crate) fn reason_as_str(reason: &EvaluationReason) -> &'static str {
    match reason {
        EvaluationReason::Unknown => "unknown",
        EvaluationReason::FlagDisabled => "flag disabled",
        EvaluationReason::Match => "match",
        EvaluationReason::Default => "default",
    }
}
