use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use near_accounts::accounts::get_account_balance;
use near_primitives::types::AccountId;
use near_providers::JsonRpcProvider;
use std::sync::Arc;
use std::time::Instant;

async fn get_balance(account_id: web::Path<String>) -> impl Responder {
    let provider = Arc::new(JsonRpcProvider::new("https://rpc.mainnet.near.org"));
    match account_id.parse::<AccountId>() {
        Ok(account_id) => {
            let start_time = Instant::now(); // Start timing before the call
            let result = get_account_balance(provider, account_id.clone()).await;
            let duration = start_time.elapsed(); // Measure the duration after the call

            match result {
                Ok(balance) => {
                    // Include duration in the response or log it
                    let response = format!(
                        "Account ID: {}\nTotal: {}\nState Staked: {}\nStaked: {}\nAvailable: {}\nTime taken: {:.2?}",
                        account_id, balance.total, balance.state_staked, balance.staked, balance.available, duration
                    );
                    HttpResponse::Ok().content_type("text/plain").body(response)
                },
                Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
            }
        },
        Err(_) => HttpResponse::BadRequest().body("Invalid account ID"),
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .route("/balance/{account_id}", web::get().to(get_balance))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
