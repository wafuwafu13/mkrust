use crate::error::*;
use crate::hosts::app::find_hosts;
use crate::mackerelclient::new::new;

pub async fn do_hosts() -> Result<Vec<mackerel_client::host::Host>> {
    let client = match new() {
        Ok(client) => client,
        Err(e) => return Err(e),
    };

    match find_hosts(client).await {
        Ok(hosts) => return Ok(hosts),
        Err(e) => return Err(e),
    }
}
