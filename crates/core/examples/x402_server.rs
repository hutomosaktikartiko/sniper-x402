use axum::{Json, Router, routing::get};
use serde_json::json;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use x402_axum::{IntoPriceTag, X402Middleware};
use x402_rs::{
    address_sol,
    network::{Network, USDCDeployment},
};

#[tokio::main]
async fn main() {
    // middleware
    let x402 = X402Middleware::try_from("https://facilitator.x402.rs")
        .unwrap()
        .with_base_url("http://localhost:3000/".parse().unwrap());

    let usdc = USDCDeployment::by_network(Network::SolanaDevnet)
        .pay_to(address_sol!("EGBQqKn968sVv5cQh5Cr72pSTHfxsuzq7o7asqYB5uEV"));

    let app = Router::new().route("/premium", get(premium_handler)).layer(
        x402.with_description("Premium API")
            .with_price_tag(usdc.amount(0.00025).unwrap()),
    );

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("x402 server is running on {}", addr);
    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn premium_handler() -> Json<serde_json::Value> {
    Json(json!({
        "message": "Welcome to the Premium API!",
        "data": "This content costs 0.00025 USDC."
    }))
}
