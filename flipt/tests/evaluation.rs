use flipt::NoneAuthentication;
use open_feature::EvaluationContext;
use openfeature_flipt::open_feature::provider::FeatureProvider;
use openfeature_flipt::provider::{Config, FliptProvider};

#[tokio::test]
async fn test_bool() {
    let config = Config {
        endpoint: "http://localhost:8080/".to_string(),
        auth_strategy: NoneAuthentication::new(),
        timeout: 60,
    };
    let ctx = EvaluationContext {
        targeting_key: None,
        custom_fields: Default::default(),
    };

    let provider = FliptProvider::new(config).unwrap();
    let details = provider
        .resolve_bool_value("flag_boolean", &ctx)
        .await
        .unwrap();
    assert!(details.value);
}

/**

curl --request POST \
  --url http://localhost:8080/api/v1/namespaces/default/variant \
  --header 'Content-Type: application/json' \
  --header 'Accept: application/json' \
  --data '{
  "context": {},
  "entityId": "entity_id",
  "flagKey": "variant1",
  "namespaceKey": "namespacekey"
}'

curl --request POST \
  --url http://localhost:8080/api/v1/namespaces/default/boolean \
  --header 'Content-Type: application/json' \
  --header 'Accept: application/json' \
  --data '{
  "context": {},
  "entityId": "entity_id",
  "flagKey": "flag_boolean",
  "namespaceKey": "namespacekey"
}'

curl --request POST \
  --url http://localhost:8080/api/v1/namespaces/default/boolean \
  --header 'Content-Type: application/json' \
  --header 'Accept: application/json' \
  --data '{
  "context": {},
  "entityId": "entity_id",
  "flagKey": "flag_boolean",
}'

 */

#[tokio::test]
async fn test_struct() {
    let config = Config {
        endpoint: "http://localhost:8080/".to_string(),
        auth_strategy: NoneAuthentication::new(),
        timeout: 60,
    };
    let ctx = EvaluationContext {
        targeting_key: None,
        custom_fields: Default::default(),
    };

    let provider = FliptProvider::new(config).unwrap();
    let details = provider
        .resolve_struct_value("variant1", &ctx)
        .await
        .unwrap();
    println!("{:?}", details);
    assert!(false)
}
