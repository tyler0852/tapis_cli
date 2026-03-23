use crate::client::Client;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AuthHelloResponse {
    pub message: String,
    // pub result: String,
    pub status: String,
    pub version: String,
    // pub metadata: serde_json::Value,
}

pub fn hello(client: &Client) -> Result<AuthHelloResponse, Box<dyn std::error::Error>> {
    let url = format!("{}/v3/oauth2/hello", client.base_url.trim_end_matches('/'));
    let resp = reqwest::blocking::get(&url)?;
    let text = resp.text()?;
    let body: AuthHelloResponse = serde_json::from_str(&text)?;
    Ok(body)
}