use reqwest::{Client, Response, header::HeaderMap};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct AdGuardFilteringStatus {
    pub filters: Vec<Filter>,
}

#[derive(Deserialize)]
pub struct Filter {
    pub url: String,
    pub name: String,
    pub rules_count: u32,
    pub enabled: bool,
}

pub async fn fetch_adguard_filter_list(
    client: &Client,
    endpoint: &str,
    username: &str,
    password: &str,
) -> Result<AdGuardFilteringStatus, reqwest::Error> {
    let url = format!("{}/control/filtering/status", endpoint);

    let auth_string = format!("{}:{}", username, password);
    let auth_header_value = format!("Basic {}", base64::encode(&auth_string));
    let mut headers = HeaderMap::new();
    headers.insert("Authorization", auth_header_value.parse().unwrap());

    let res: Response = client.get(&url).headers(headers).send().await?;
    let status: AdGuardFilteringStatus = res.json().await?;

    Ok(status)
}
