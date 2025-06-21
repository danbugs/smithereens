// Integration test that hits the *real* start.gg API.
// Run with: `cargo test -- --ignored`  (skipped by default to avoid CI failures / rate‑limits)
// Requires STARTGG_TOKEN env var to be set.

use backend::startgg::StartggClient;
use serde_json::Value;

#[tokio::test]
#[ignore]
async fn live_search_returns_nodes() {
    dotenvy::dotenv().ok();
    let client = StartggClient::from_env().expect("env vars");

    // Query a well‑known tag – should return at least one node.
    let json: Value = client
        .search_players("MkLeo", 1, 25)
        .await
        .expect("live call");
    let nodes = &json["data"]["players"]["nodes"];
    println!("live json nodes = {}", nodes);
    assert!(nodes.is_array(), "nodes should be array");
    assert!(
        !nodes.as_array().unwrap().is_empty(),
        "expected at least one player node"
    );
}

// test get user_profile
#[tokio::test]
#[ignore]
async fn live_user_profile_returns_data() {
    dotenvy::dotenv().ok();
    let client = StartggClient::from_env().expect("env vars");
    let json: Value = client.user_profile("566b1fb5", 1).await.expect("live call");
    let data = &json["data"]["user"];
    println!("live user profile data = {}", data);
    assert!(data.is_object(), "data should be an object");
    assert!(data.get("id").is_some(), "user id should be present");
}
