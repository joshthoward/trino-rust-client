mod common;

use serde::Deserialize; // TODO: Maybe re-export trait in crate
use serde_json::Value;

#[tokio::test]
async fn test_query_typed() {
    common::initialize().await;

    #[derive(Debug, Deserialize)]
    struct Nation {
        nationkey: u32,
        name: String,
        regionkey: u32,
        comment: String,
    }

    let client = trino::Client::new("http://localhost", 8080, "user");
    let res: Vec<Nation> = client
        .query("SELECT * FROM tpch.tiny.nation")
        .await
        .unwrap();
    assert_eq!(res.len(), 25);
}

#[tokio::test]
async fn test_query_untyped() {
    common::initialize().await;

    let client = trino::Client::new("http://localhost", 8080, "user");
    let res: Vec<Value> = client
        .query("SELECT * FROM tpch.tiny.nation")
        .await
        .unwrap();
    assert_eq!(res.len(), 25);
}
