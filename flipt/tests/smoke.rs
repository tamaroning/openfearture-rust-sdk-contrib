use flipt::NoneAuthentication;
use open_feature::EvaluationContext;
use openfeature_flipt::open_feature::provider::FeatureProvider;
use openfeature_flipt::provider::{Config, FliptProvider};

#[tokio::test]
async fn test_bool() {
    let config = Config {
        endpoint: "http://localhost:8080".to_string(),
        auth_strategy: NoneAuthentication::new(),
        timeout: 1000,
    };
    let ctx = EvaluationContext {
        targeting_key: None,
        custom_fields: Default::default(),
    };

    let provider = FliptProvider::new(config).unwrap();
    let details = provider.resolve_bool_value("bbboool", &ctx).await.unwrap();
    println!("{:?}", details);
    assert!(details.value);
}

#[tokio::test]
async fn test_struct() {
    let config = Config {
        endpoint: "http://localhost:8080".to_string(),
        auth_strategy: NoneAuthentication::new(),
        timeout: 1000,
    };
    let ctx = EvaluationContext {
        targeting_key: None,
        custom_fields: Default::default(),
    };

    let provider = FliptProvider::new(config).unwrap();
    let details = provider.resolve_struct_value("a", &ctx).await.unwrap();
    println!("{:?}", details);
    assert!(false)
}
