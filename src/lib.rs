use anyhow::Result;
use models::BalanceResponse;
use reqwest::Client;
use serde_json::json;

pub mod models;

pub struct Solver {
    client_key: String,
    client: Client,
}

impl Solver {
    pub fn new<T>(key: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            client_key: key.into(),
            client: Client::new(),
        }
    }

    pub async fn get_balance(&self) -> Result<BalanceResponse> {
        let data = json!({
            "clientKey": self.client_key,
        });
        let request = self
            .client
            .post("https://capbypass.com/api/getBalance")
            .header("content-type", "application/json")
            .json(&data)
            .send()
            .await?;
        let response = request.error_for_status()?;
        let response: BalanceResponse = response.json().await?;

        Ok(response)
    }
}
