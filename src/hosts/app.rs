use mackerel_client::*;

pub async fn find_hosts(
    client: Client,
) -> Result<Vec<mackerel_client::host::Host>, mackerel_client::error::Error> {
    return client.list_hosts().await;
}
