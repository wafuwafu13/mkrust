use crate::hosts::app::find_hosts;
use crate::mackerelclient::new::new;
use crate::{error::*, format::format::Host};

pub async fn do_hosts() -> Result<Vec<Host>> {
    let client = match new() {
        Ok(client) => client,
        Err(e) => return Err(e),
    };

    match find_hosts(client).await {
        Ok(hosts) => return Ok(hosts),
        Err(e) => return Err(e),
    }
}
