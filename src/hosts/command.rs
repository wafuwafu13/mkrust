use crate::hosts::app::find_hosts;
use crate::mackerelclient::new::new;
use std::env::VarError;

pub async fn do_hosts(
) -> Result<Result<Vec<mackerel_client::host::Host>, mackerel_client::error::Error>, VarError> {
    let client = match new() {
        Ok(client) => client,
        Err(e) => return Err(e),
    };

    Ok(find_hosts(client).await)
}
