use reqwest::{
  header::{HeaderValue, CONTENT_LENGTH, AUTHORIZATION},
};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize, Clone)]
pub struct DomainData {
    pub name: String,
    pub count: i32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct StatsResponse {
    pub num_dns_queries: u64,
    pub num_blocked_filtering: u64,
    pub num_replaced_safebrowsing: u64,
    pub num_replaced_safesearch: u64,
    pub num_replaced_parental: u64,
    pub avg_processing_time: f64,
    pub dns_queries: Vec<u64>,
    pub blocked_filtering: Vec<u64>,
    pub replaced_safebrowsing: Vec<u64>,
    pub replaced_parental: Vec<u64>,

    #[serde(default, skip_deserializing)]
    pub dns_queries_chart: Vec<(f64, f64)>,
    #[serde(default, skip_deserializing)]
    pub blocked_filtering_chart: Vec<(f64, f64)>,

    #[serde(rename = "top_queried_domains", deserialize_with = "deserialize_domains")]
    pub top_queried_domains: Vec<DomainData>,
    #[serde(rename = "top_blocked_domains", deserialize_with = "deserialize_domains")]
    pub top_blocked_domains: Vec<DomainData>,
    #[serde(rename = "top_clients", deserialize_with = "deserialize_domains")]
    pub top_clients: Vec<DomainData>,
}

pub async fn fetch_adguard_stats(
    client: &reqwest::Client,
    endpoint: &str,
    username: &str,
    password: &str,
) -> Result<StatsResponse, anyhow::Error> {
    let auth_string = format!("{}:{}", username, password);
    let auth_header_value = format!("Basic {}", base64::encode(&auth_string));
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(AUTHORIZATION, auth_header_value.parse()?);
    headers.insert(CONTENT_LENGTH, HeaderValue::from_static("0"));

    let url = format!("{}/control/stats", endpoint);
    let response = client.get(&url).headers(headers).send().await?;
    if !response.status().is_success() {
        return Err(anyhow::anyhow!("Request failed with status code {}", response.status()));
    }

    let data = response.json().await?;
    Ok(data)
}


fn deserialize_domains<'de, D>(deserializer: D) -> Result<Vec<DomainData>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let raw_vec: Vec<HashMap<String, i32>> = serde::Deserialize::deserialize(deserializer)?;
    Ok(raw_vec
        .into_iter()
        .flat_map(|mut map| {
            map.drain().map(|(name, count)| DomainData { name, count }).collect::<Vec<_>>()
        })
        .collect())
}

