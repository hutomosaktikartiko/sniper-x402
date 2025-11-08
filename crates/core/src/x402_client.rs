use reqwest::{Client, Response};
use reqwest_middleware::ClientWithMiddleware;
use serde::Serialize;
use solana_client::rpc_client::RpcClient;
use solana_sdk::signature::Keypair;
use std::error::Error;
use x402_reqwest::{
    MaxTokenAmountFromAmount, ReqwestWithPayments, ReqwestWithPaymentsBuild,
    chains::solana::SolanaSenderWallet,
};
use x402_rs::network::{Network, USDCDeployment};

#[derive(Clone)]
pub struct X402Client {
    client: ClientWithMiddleware,
    max_usdc_per_day: f64,
}

impl X402Client {
    pub fn new(
        keypair: Keypair,
        rpc_url: &str,
        max_usdc_per_day: f64,
    ) -> Result<Self, Box<dyn Error>> {
        let rpc_client = RpcClient::new(rpc_url.to_string());
        let sender = SolanaSenderWallet::new(keypair, rpc_client);

        let usdc = if rpc_url.contains("devnet") {
            USDCDeployment::by_network(Network::SolanaDevnet)
        } else {
            USDCDeployment::by_network(Network::Solana)
        };

        let client = Client::new()
            .with_payments(sender)
            .prefer(usdc.clone())
            .max(usdc.amount(max_usdc_per_day)?)
            .build();

        Ok(Self {
            client: client,
            max_usdc_per_day,
        })
    }

    pub fn from_session(
        session_key: &[u8],
        rpc_url: &str,
        max_usdc_per_day: f64,
    ) -> Result<Self, Box<dyn Error>> {
        let keypair = Keypair::try_from(session_key).map_err(|_| "Invalid session key")?;
        Self::new(keypair, rpc_url, max_usdc_per_day)
    }

    pub async fn get(&self, url: &str) -> Result<Response, Box<dyn Error>> {
        let response = self.client.get(url).send().await?;
        Ok(response)
    }

    pub async fn post(&self, url: &str) -> Result<Response, Box<dyn Error>> {
        let response = self.client.post(url).send().await?;
        Ok(response)
    }

    pub async fn post_json<T: Serialize>(
        &self,
        url: &str,
        body: T,
    ) -> Result<Response, Box<dyn Error>> {
        let body_bytes = serde_json::to_vec(&body)?;
        let response = self
            .client
            .post(url)
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .body(body_bytes)
            .send()
            .await?;
        Ok(response)
    }
}
