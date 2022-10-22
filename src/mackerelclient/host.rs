use crate::mackerelclient::env;
use serde_derive::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Host {
    pub id: String,
    pub name: String,
    pub created_at: u64,
    pub size: String,
    pub status: HostStatus,
    pub memo: String,
    pub is_retired: bool,
    pub retired_at: Option<u64>,
    pub roles: HashMap<String, Vec<String>>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum HostStatus {
    Working,
    Standby,
    Maintenance,
    Poweroff,
}

#[derive(Deserialize)]
struct ListHostsResponse {
    hosts: Vec<Host>,
}

pub async fn list_hosts() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let resp = client
        .get("https://api.mackerelio.com/api/v0/hosts")
        // TODO: panicked at 'called `Result::unwrap()` on an `Err` value: EnvError(NotPresent)'
        .header("X-Api-Key", env::get_apikey().unwrap())
        .send()
        .await?
        .json()
        .await
        .map(|res: ListHostsResponse| res.hosts);

    println!("{:?}", resp);
    return Ok(());
}
