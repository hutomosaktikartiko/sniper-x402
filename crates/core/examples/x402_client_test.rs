use core::{X402Client, wallet::load_keypair_from_file};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // read keypair
    let keypair = load_keypair_from_file("wallets/1.json")?;

    // create x402 client
    let client = X402Client::new(keypair, "https://api.devnet.solana.com", 1.0)?;

    // call premium api
    let resp = client.get("http://localhost:3000/premium").await?;

    println!("Status: {}", resp.status());
    println!("Body: {}", resp.text().await?);

    Ok(())
}
