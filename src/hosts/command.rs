use crate::hosts::app::find_hosts;
use crate::{error::*, format::format::Host};

pub async fn do_hosts() -> Result<Vec<Host>> {
    match find_hosts().await {
        Ok(hosts) => return Ok(hosts),
        Err(e) => return Err(e),
    }
}
