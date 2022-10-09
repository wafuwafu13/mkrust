use crate::error::*;
use mackerel_client::*;

pub async fn find_hosts(client: Client) -> Result<Vec<mackerel_client::host::Host>> {
    match client.list_hosts().await {
        Ok(hosts) => return Ok(hosts),
        Err(e) => return Err(Error::ClientError(e)),
    }
}
