use reqwest::{
  header::{HeaderValue, CONTENT_LENGTH, AUTHORIZATION},
};
use serde::Deserialize;

/// Represents the status response from the AdGuard Home API.
///
/// This struct is used to deserialize the JSON response from the 
/// `/control/status` endpoint.
///
/// # Example
///
/// A `StatusResponse` is typically obtained like this:
///
/// ```
/// let client = reqwest::Client::new();
/// let hostname = "http://localhost:3000";
/// let username = "username";
/// let password = "password";
/// let status = fetch_adguard_status(&client, &hostname, &username, &password).await?;
/// println!("AdGuard Status: {:?}", status);
/// ```
///
/// # Fields
///
/// * `version` - The version of the AdGuard Home instance.
/// * `language` - The language currently used in the AdGuard Home instance.
/// * `dns_addresses` - The DNS addresses used by the AdGuard Home instance.
/// * `dns_port` - The port number on which the DNS server is running.
/// * `http_port` - The port number on which the HTTP server is running.
/// * `protection_disabled_duration` - The duration for which protection is disabled (in seconds).
/// * `protection_enabled` - Whether or not protection is currently enabled.
/// * `dhcp_available` - Whether or not DHCP is available.
/// * `running` - Whether or not the AdGuard Home instance is currently running.
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


/// Fetches the current status from the AdGuard Home instance.
///
/// This function sends a GET request to the `/control/status` endpoint of the 
/// AdGuard Home API, then deserializes the JSON response into a `StatusResponse`.
///
/// # Arguments
///
/// * `client` - A reference to the `reqwest::Client`.
/// * `hostname` - The hostname of the AdGuard Home instance.
/// * `username` - The username for the AdGuard Home instance.
/// * `password` - The password for the AdGuard Home instance.
///
/// # Returns
///
/// A `Result` which is `Ok` if the status was successfully fetched and `Err` otherwise.
/// The `Ok` variant contains a `StatusResponse`.
///
/// # Example
///
/// ```
/// let client = reqwest::Client::new();
/// let hostname = "http://localhost:80";
/// let username = "username";
/// let password = "password";
/// let status = fetch_adguard_status(&client, &hostname, &username, &password).await?;
/// println!("AdGuard Status: {:?}", status);
/// ```
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
