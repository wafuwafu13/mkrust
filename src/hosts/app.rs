use crate::{error::*, format::format::Host};
use mackerel_client::*;

pub async fn find_hosts(client: Client) -> Result<Vec<Host>> {
    match client.list_hosts().await {
        Ok(hosts) => {
            let mut hosts_format: Vec<Host> = Vec::new();
            for host in hosts.iter() {
                let display_name = match &host.display_name {
                    Some(display_name) => display_name.to_string(),
                    None => "".to_string(),
                };
                hosts_format.push(Host {
                    id: host.id.to_string(),
                    name: host.name.to_string(),
                    display_name,
                    status: host.status.to_string(),
                    memo: host.memo.to_string(),
                    is_retired: host.is_retired,
                })
            }
            return Ok(hosts_format);
        }
        Err(e) => return Err(Error::ClientError(e)),
    }
}
