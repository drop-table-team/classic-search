use anyhow::bail;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize)]
struct RegisterPayload {
    name: String,
    address: String,
}

#[derive(Clone, Deserialize)]
pub struct RegisterResponse {
    pub mongo_address: String,
    pub mongo_database: String,
    pub mongo_collection: String,
    pub qdrant_address: String,
}

#[derive(Clone, Debug, Serialize)]
struct UnregisterPayload {}

pub struct Module {
    name: String,
    address: String,
    client: Client,
}

impl Module {
    pub fn new<A: AsRef<str>>(address: A, name: String) -> Self {
        Module {
            address: address.as_ref().to_string(),
            name,
            client: Client::new(),
        }
    }

    pub async fn register<A: AsRef<str>>(&self, address: A) -> anyhow::Result<RegisterResponse> {
        let url = format!("{}/modules/output/register", address.as_ref().to_string());

        let payload = RegisterPayload {
            name: self.name.clone(),
            address: self.address.clone(),
        };

        let response = self
            .client
            .post(&url)
            .body(serde_json::to_string(&payload).unwrap())
            .send()
            .await?;

        if response.status() != 200 {
            bail!(
                "Couldn't register module '{}' on backend '{}', got status {}: {}",
                self.name,
                address.as_ref(),
                response.status(),
                String::from_utf8_lossy(&response.bytes().await?)
            );
        }

        Ok(response.json::<RegisterResponse>().await?)
    }

    pub async fn unregister<A: AsRef<str>>(&mut self, address: A) -> anyhow::Result<()> {
        let url = format!("{}/modules/output/unregister", address.as_ref().to_string());

        let payload = UnregisterPayload {};

        self.client
            .post(&url)
            .body(serde_json::to_string(&payload).unwrap())
            .send()
            .await?;

        Ok(())
    }
}
