// Hammers StartGG with many concurrent `getEventId` queries until a rate‑limit error appears.
// Ignored by default — run with: `cargo test -- --ignored live_rate_limit`.

use backend::{StartggClient, StartggError};
use futures::future::join_all;

const SLUG: &str = "tournament/genesis-9-1/event/ultimate-singles";
const BATCH: usize = 250; // concurrent requests per wave
const WAVES: usize = 4; // total waves => 1 000 requests max

type Res = Result<serde_json::Value, StartggError>;

#[tokio::test]
#[ignore]
async fn live_rate_limit() {
    dotenvy::dotenv().ok();
    let client = StartggClient::from_env().expect("env vars");

    for w in 1..=WAVES {
        let futures = (0..BATCH).map(|_| client.get_event_id(SLUG));
        let results: Vec<Res> = join_all(futures).await;
        if results
            .iter()
            .any(|r| matches!(r, Err(StartggError::RateLimited)))
        {
            println!("✔ hit rate limit in wave {w}");
            return;
        }
        println!("wave {w}: still no rate limit");
    }
    panic!("did not hit rate limit after {} requests", BATCH * WAVES);
}
