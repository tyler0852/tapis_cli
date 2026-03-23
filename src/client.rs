// This file defines the Client struct, which is used to store the base url for the API and construct endpoint urls.
pub struct Client {
    pub base_url: String,
}

// Implementation of the Client struct
impl Client {
    pub fn new(base_url: String) -> Self {
        Self { base_url }
    }

    pub fn apps_healthcheck_url(&self) -> String {
        format!("{}/v3/apps/healthcheck", self.base_url.trim_end_matches('/'))
    }

    pub fn auth_hello_url(&self) -> String {
        format!("{}/v3/oauth2/hello", self.base_url.trim_end_matches('/'))
    }

    pub fn auth_device_code_url(&self) -> String {
        format!("{}/v3/oauth2/device/code", self.base_url.trim_end_matches('/'))
    }

    pub fn auth_tokens_url(&self) -> String {
        format!("{}/v3/oauth2/tokens", self.base_url.trim_end_matches('/'))
    }
}