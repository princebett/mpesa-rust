use dotenv;
use mpesa::{Environment, Mpesa};
use std::env;

#[tokio::test]
async fn c2b_simulate_test() {
    dotenv::dotenv().ok();

    let client = Mpesa::new(
        env::var("CLIENT_KEY").unwrap(),
        env::var("CLIENT_SECRET").unwrap(),
        Environment::Sandbox,
    );

    let response = client
        .c2b_simulate()
        .short_code("600496")
        .msisdn("254700000000")
        .amount(1000)
        .send()
        .await;

    assert!(response.is_ok())
}
