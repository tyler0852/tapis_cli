use crate::client::Client;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct HealthcheckResponse {
    pub status: String,
    pub message: String,
    pub result: String,
    // pub version: String,
    // pub commit: String,
    // pub build: String,
    // pub metadata: Option<serde_json::Value>,
}

pub fn healthcheck(client: &Client) -> Result<HealthcheckResponse, Box<dyn std::error::Error>> {
    let url = client.apps_healthcheck_url();
    let resp = reqwest::blocking::get(&url)?;
    let text = resp.text()?;
    let body: HealthcheckResponse = serde_json::from_str(&text)?;
    Ok(body)
}