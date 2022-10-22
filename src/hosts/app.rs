use crate::{error::*, format::format::Host, mackerelclient::host::list_hosts};

pub async fn find_hosts() -> Result<Vec<Host>> {
    match list_hosts().await {
        Ok(hosts) => {
            let mut hosts_format: Vec<Host> = Vec::new();
            for host in hosts.iter() {
                hosts_format.push(Host {
                    id: host.id.to_string(),
                    name: host.name.to_string(),
                    memo: host.memo.to_string(),
                    is_retired: host.is_retired,
                })
            }
            return Ok(hosts_format);
        }
        Err(e) => return Err(Error::RequestError(e)),
    }
}
