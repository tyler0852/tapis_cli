use crate::client::Client;
use serde_json::Value;
use std::thread::sleep;
use std::time::Duration;

const CLIENT_ID: &str = "tapis_cli"; // My made up client for this app
const DEVICE_CODE_GRANT_TYPE: &str = "device_code"; // Only want to support device code flow here
const DEVICE_CODE_POLL_INTERVAL: u64 = 3; // seconds

pub fn get_token_flow(client: &Client) -> Result<(), Box<dyn std::error::Error>> {
    let device_body = request_device_code(client)?; // Stores the output from the device code endpoint
    let result = &device_body["result"]; // Graps only the result field 

    // Extracts the fields we need from the device code response, and returns an error if any of them are missing
    let device_code = result["device_code"]
        .as_str()
        .ok_or("missing device_code in response")?;
    let user_code = result["user_code"]
        .as_str()
        .ok_or("missing user_code in response")?;
    let verification_uri = result["verification_uri"]
        .as_str()
        .ok_or("missing verification_uri in response")?;

    // Prints instructions for the user to go to the verification uri and enter the user code
    println!("");
    println!("To authorize this application, please follow these steps:");
    println!("Go to: {}", verification_uri);
    println!("Enter this code: {}", user_code);
    println!("Waiting for authorization...");

    loop {
        let token_body = poll_for_token(client, device_code)?;

        let status = token_body["status"].as_str().unwrap_or("");
        let message = token_body["message"].as_str().unwrap_or("");

        if status == "success" {
            let access_token = token_body["result"]["access_token"]["access_token"]
                .as_str()
                .ok_or("missing access token in response")?;
            let access_expires_at = token_body["result"]["access_token"]["expires_at"]
                .as_str()
                .ok_or("missing access token expires_at in response")?;
            let access_expires_in = token_body["result"]["access_token"]["expires_in"]
                .as_i64()
                .ok_or("missing access token expires_in in response")?;
            let refresh_token = token_body["result"]["refresh_token"]["refresh_token"]
                .as_str()
                .ok_or("missing refresh token in response")?;
            let refresh_expires_at = token_body["result"]["refresh_token"]["expires_at"]
                .as_str()
                .ok_or("missing refresh token expires_at in response")?;
            let refresh_expires_in = token_body["result"]["refresh_token"]["expires_in"]
                .as_i64()
                .ok_or("missing refresh token expires_in in response")?;

            println!("");
            println!("Authorization successful.");
            println!("");
            println!("access_token: {}", access_token);
            println!("");
            println!("access_token_expires_at: {}", access_expires_at);
            println!("");
            println!("access_token_expires_in: {}", access_expires_in);
            println!("");
            println!("refresh_token: {}", refresh_token);
            println!("");
            println!("refresh_token_expires_at: {}", refresh_expires_at);
            println!("");
            println!("refresh_token_expires_in: {}", refresh_expires_in);
            println!("");
            return Ok(());
        }

        if message == "device code not ready." {
            println!("...");
            sleep(Duration::from_secs(DEVICE_CODE_POLL_INTERVAL));
            continue;
        }

        return Err(format!("token polling failed: {}", message).into());
    }

}

// Calls the device code endpoint and returns the response body as a serde_json::Value
fn request_device_code(client: &Client) -> Result<Value, Box<dyn std::error::Error>> {
    let url = client.auth_device_code_url();

    let resp = reqwest::blocking::Client::new()
        .post(&url)
        .json(&serde_json::json!({
            "client_id": CLIENT_ID
        }))
        .send()?;

    let text = resp.text()?;
    let body: Value = serde_json::from_str(&text)?;
    Ok(body)
}

// Calls the tokens endpoint with the device code and returns the response body as a serde_json::Value
fn poll_for_token(client: &Client, device_code: &str) -> Result<Value, Box<dyn std::error::Error>> {
    let url = client.auth_tokens_url();

    let resp = reqwest::blocking::Client::new()
        .post(&url)
        .json(&serde_json::json!({
            "grant_type": DEVICE_CODE_GRANT_TYPE,
            "device_code": device_code,
            "client_id": CLIENT_ID
        }))
        .send()?;

    let text = resp.text()?;
    let body: Value = serde_json::from_str(&text)?;
    Ok(body)
}