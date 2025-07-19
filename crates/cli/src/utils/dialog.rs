use crate::error::{PortoError, Result};
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Serialize, Deserialize)]
pub struct DialogRequest {
    pub method: String,
    pub params: serde_json::Value,
    pub id: u64,
}

pub struct DialogBuilder {
    host: String,
    relay_url: String,
}

impl DialogBuilder {
    pub fn new(host: String) -> Self {
        Self {
            host,
            relay_url: String::new(),
        }
    }

    pub fn set_relay_url(&mut self, relay_url: String) {
        self.relay_url = relay_url;
    }

    pub fn build_url(&self, request: &DialogRequest) -> Result<String> {
        let base_url = format!("https://{}/dialog/{}", self.host, request.method);
        let mut url = Url::parse(&base_url)
            .map_err(|e| PortoError::AccountCreation(format!("Invalid URL: {e}")))?;

        let params = [
            ("id", request.id.to_string()),
            ("method", request.method.clone()),
            ("params", serde_json::to_string(&request.params)?),
            (
                "referrer",
                serde_json::to_string(&serde_json::json!({
                    "title": "Porto CLI",
                    "url": "cli://porto"
                }))?,
            ),
            ("relayUrl", self.relay_url.clone()),
        ];

        url.query_pairs_mut().extend_pairs(params.iter());

        Ok(url.to_string())
    }

    pub async fn open_dialog(&self, url: &str) -> Result<()> {
        webbrowser::open(url)
            .map_err(|e| PortoError::AccountCreation(format!("Failed to open browser: {e}")))?;
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConnectParams {
    #[serde(rename = "createAccount")]
    pub create_account: bool,
    #[serde(rename = "grantAdmins", skip_serializing_if = "Option::is_none")]
    pub grant_admins: Option<Vec<AdminKeyGrant>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AdminKeyGrant {
    #[serde(rename = "publicKey")]
    pub public_key: String,
    #[serde(rename = "type")]
    pub key_type: String,
}
