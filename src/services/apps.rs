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
    let url = client.apps_healthcheck_url(); // builds the url using client.rs
    let resp = reqwest::blocking::get(&url)?.error_for_status()?; // makes the request to Tapis and checks for http errors
    let text = resp.text()?; // takes the JSON response from Tapis and turns it into a string
    let body: HealthcheckResponse = serde_json::from_str(&text)?; // takes the string and deserializes it into a HealthcheckResponse struct
    Ok(body) // returns the HealthcheckResponse struct
}