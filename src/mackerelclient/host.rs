use std::collections::HashMap;

pub async fn list_hosts() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://api.mackerelio.com/api/v0/hosts")
        .await?
        .json::<HashMap<String, String>>()
        .await?;

    println!("{:?}", resp);
    return Ok(());
}
