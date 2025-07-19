use crate::error::{PortoError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use url::Url;

#[derive(Debug, Serialize, Deserialize)]
pub struct DialogRequest {
    pub method: String,
    pub params: serde_json::Value,
    pub id: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DialogResponse {
    pub result: Option<serde_json::Value>,
    pub error: Option<serde_json::Value>,
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
            relay_url: String::new() 
        }
    }
    
    pub fn set_relay_url(&mut self, relay_url: String) {
        self.relay_url = relay_url;
    }
    
    pub fn host(&self) -> &str {
        &self.host
    }
    
    pub fn relay_url(&self) -> &str {
        &self.relay_url
    }

    pub fn build_url(&self, request: &DialogRequest) -> Result<String> {
        let base_url = format!("https://{}/dialog/{}", self.host, request.method);
        let mut url = Url::parse(&base_url)
            .map_err(|e| PortoError::DialogError(format!("Invalid URL: {}", e)))?;

        let params = vec![
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

        url.query_pairs_mut()
            .extend_pairs(params.iter());

        Ok(url.to_string())
    }

    pub async fn open_dialog(&self, url: &str) -> Result<()> {
        webbrowser::open(url)
            .map_err(|e| PortoError::DialogError(format!("Failed to open browser: {}", e)))?;
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Account {
    pub address: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConnectResult {
    pub accounts: Vec<Account>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddFundsParams {
    pub address: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PrepareCallsParams {
    pub calls: Vec<serde_json::Value>,
    pub key: crate::utils::AdminKey,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PrepareCallsResult {
    pub digest: String,
    #[serde(flatten)]
    pub request: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SendPreparedCallsParams {
    pub signature: String,
    #[serde(flatten)]
    pub request: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CallResult {
    pub id: String,
}