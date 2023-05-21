use reqwest::{
  header::{HeaderValue, CONTENT_LENGTH, AUTHORIZATION},
};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct StatusResponse {
    pub version: String,
    pub language: String,
    pub dns_addresses: Vec<String>,
    pub dns_port: u16,
    pub http_port: u16,
    pub protection_disabled_duration: u64,
    pub protection_enabled: bool,
    pub dhcp_available: bool,
    pub running: bool,
}

pub async fn fetch_adguard_status(
    client: &reqwest::Client,
    endpoint: &str,
    username: &str,
    password: &str,
) -> Result<StatusResponse, anyhow::Error> {
    let auth_string = format!("{}:{}", username, password);
    let auth_header_value = format!("Basic {}", base64::encode(&auth_string));
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(AUTHORIZATION, auth_header_value.parse()?);
    headers.insert(CONTENT_LENGTH, HeaderValue::from_static("0"));

    let url = format!("{}/control/status", endpoint);
    let response = client.get(&url).headers(headers).send().await?;
    if !response.status().is_success() {
        return Err(anyhow::anyhow!("Request failed with status code {}", response.status()));
    }

    let data = response.json().await?;
    Ok(data)
}
