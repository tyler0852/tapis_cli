use crate::client::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
//
pub struct AuthHelloResponse {
    pub message: String,
    // pub result: String,
    pub status: String,
    pub version: String,
    // pub metadata: serde_json::Value,
}

#[derive(Debug, Serialize)]
pub struct NewDeviceCodeRequest {
    pub client_id: String,
}

#[derive(Debug, Deserialize)]
pub struct DeviceCodeResult {
    pub device_code: String,
    pub user_code: String,
    pub client_id: String,
    pub expires_in: String,
    pub verification_uri: String,
}

#[derive(Debug, Deserialize)]
pub struct GenerateDeviceCodeResponse {
    pub message: String,
    pub result: DeviceCodeResult,
    pub status: String,
    pub version: String,
}

#[derive(Debug, Serialize)]
pub struct NewTokenRequest {
    pub username: Option<String>,
    pub password: Option<String>,
    pub client_id: Option<String>,
    pub client_key: Option<String>,
    pub grant_type: String,
    pub redirect_uri: Option<String>,
    pub code: Option<String>,
    pub device_code: Option<String>,
    pub refresh_token: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct AccessTokenInfo {
    pub access_token: String,
    pub id_token: String,
    pub expires_at: String,
    pub expires_in: i32,
    pub jti: String,
}

#[derive(Debug, Deserialize)]
pub struct RefreshTokenInfo {
    pub refresh_token: String,
    pub expires_at: String,
    pub expires_in: i32,
    pub jti: String,
}

#[derive(Debug, Deserialize)]
pub struct TokenResult {
    pub access_token: AccessTokenInfo,
    pub refresh_token: RefreshTokenInfo,
}

#[derive(Debug, Deserialize)]
pub struct CreateTokenResponse {
    pub message: String,
    pub result: TokenResult,
    pub status: String,
    pub version: String,
}

pub fn hello(client: &Client) -> Result<AuthHelloResponse, Box<dyn std::error::Error>> {
    let url = client.auth_hello_url(); // builds the url using client.rs
    let resp = reqwest::blocking::get(&url)?.error_for_status()?; // makes the request to Tapis and checks for http errors
    let text = resp.text()?; // takes the JSON response from Tapis and turns it into a string
    let body: AuthHelloResponse = serde_json::from_str(&text)?; // takes the string and deserializes it into an AuthHelloResponse struct
    Ok(body) // returns the AuthHelloResponse struct
}

pub fn generate_device_code(
    client: &Client,
    request_body: &NewDeviceCodeRequest,
) -> Result<GenerateDeviceCodeResponse, Box<dyn std::error::Error>> {
    let url = client.auth_device_code_url(); // builds the url using client.rs
    let http_client = reqwest::blocking::Client::new(); // creates a reqwest client so we can send a POST request with a JSON body
    let resp = http_client.post(&url).json(request_body).send()?.error_for_status()?; // sends the request body to Tapis and checks for http errors
    let text = resp.text()?; // takes the JSON response from Tapis and turns it into a string
    let body: GenerateDeviceCodeResponse = serde_json::from_str(&text)?; // takes the string and deserializes it into a GenerateDeviceCodeResponse struct
    Ok(body) // returns the GenerateDeviceCodeResponse struct
}

pub fn create_token(
    client: &Client,
    request_body: &NewTokenRequest,
) -> Result<CreateTokenResponse, Box<dyn std::error::Error>> {
    let url = client.auth_tokens_url(); // builds the url using client.rs
    let http_client = reqwest::blocking::Client::new();
    let resp = http_client.post(&url).json(request_body).send()?.error_for_status()?;
    let text = resp.text()?;
    let body: CreateTokenResponse = serde_json::from_str(&text)?;
    Ok(body)
}