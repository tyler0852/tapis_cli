use crate::client::Client;
use serde::{Deserialize, Serialize};

/*
I've structured this so that the endpoint stucts are what I actually want to return
But I also have a section that has structs defined exactly how they are in the OpenAPI spec
*/

//////////////////////////
///// Hello endpoint /////
//////////////////////////

#[derive(Debug, Deserialize)]
pub struct AuthHelloResponse {
    pub message: String,
    pub metadata: serde_json::Value,
    pub result: String,
    pub status: String,
    pub version: String,
}

////////////////////////////////
///// device code endpoint /////
////////////////////////////////

#[derive(Debug, Deserialize)]
pub struct GenerateDeviceCodeResponse {
    pub message: String,
    pub metadata: serde_json::Value,
    pub result: DeviceCodeResponse,
    pub status: String,
    pub version: String,
}

///////////////////////////
///// tokens endpoint /////
///////////////////////////

#[derive(Debug, Deserialize)]
pub struct GenerateTokensResponse {
    pub message: String,
    pub metadata: serde_json::Value,
    pub result: TokenResponse,
    pub status: String,
    pub version: String,
}

////////////////////
///// Schemas //////
////////////////////


#[derive(Debug, Deserialize)]
pub struct BasicResponse {
    pub version: Option<String>,
    pub message: Option<String>,
    pub status: Option<String>,
    pub metadata: Option<serde_json::Value>,
}

// The post body request (comes straigh from NewDeviceCode schema)
#[derive(Debug, Serialize)]
pub struct NewDeviceCode {
    pub client_id: String,
}

// The response body for the device code endpoint (comes straight from DeviceCodeResponse schema)
#[derive(Debug, Deserialize)]
pub struct DeviceCodeResponse {
    pub device_code: String,
    pub user_code: String,
    pub client_id: String,
    pub expires_in: String,
    pub verification_uri: String,
}

// The post body request for the tokens endpoint (comes straight from NewToken schema)
#[derive(Debug, Serialize)]
pub struct NewToken {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,
}

// Part of the TokenResponce body
#[derive(Debug, Deserialize)]
pub struct AccessToken {
    pub access_token: String,
    pub id_token: String,
    pub expires_at: String,
    pub expires_in: i32,
    pub jti: String,
}

// Part of the TokenResponce body
#[derive(Debug, Deserialize)]
pub struct RefreshToken {
    pub refresh_token: String,
    pub expires_at: String,
    pub expires_in: i32,
    pub jti: String,
}

// The response body for the tokens endpoint (comes straight from TokenResponse schema)
#[derive(Debug, Deserialize)]
pub struct TokenResponse {
    pub access_token: AccessToken,
    pub refresh_token: Option<RefreshToken>,
}

///////////////////////////////////////
///// Functions for each endpoint /////
///////////////////////////////////////
pub fn hello(client: &Client) -> Result<AuthHelloResponse, Box<dyn std::error::Error>> {
    let url = client.auth_hello_url(); // builds the url using client.rs
    let resp = reqwest::blocking::get(&url)?.error_for_status()?; // makes the request to Tapis and checks for http errors
    let text = resp.text()?; // takes the JSON response from Tapis and turns it into a string
    let body: AuthHelloResponse = serde_json::from_str(&text)?; // takes the string and deserializes it into an AuthHelloResponse struct
    Ok(body) // returns the AuthHelloResponse struct
}

pub fn generate_device_code(client: &Client, request_body: &NewDeviceCode) -> Result<GenerateDeviceCodeResponse, Box<dyn std::error::Error>> {
    let url = client.auth_device_code_url();
    let resp = reqwest::blocking::Client::new().post(&url).json(request_body).send()?.error_for_status()?;
    let text = resp.text()?;
    let body: GenerateDeviceCodeResponse = serde_json::from_str(&text)?;
    Ok(body)
}

pub fn generate_tokens(client: &Client, request_body: &NewToken) -> Result<GenerateTokensResponse, Box<dyn std::error::Error>> {
    let url = client.auth_tokens_url();
    let resp = reqwest::blocking::Client::new().post(&url).json(request_body).send()?.error_for_status()?;
    let text = resp.text()?;
    let body: GenerateTokensResponse = serde_json::from_str(&text)?;
    Ok(body)
}
