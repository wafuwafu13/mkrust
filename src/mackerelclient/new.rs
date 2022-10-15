use crate::{error::*, mackerelclient::env};
use mackerel_client::*;

pub fn new() -> Result<Client> {
    let apikey = match env::get_apikey() {
        Ok(apikey) => apikey,
        Err(e) => return Err(e),
    };
    return Ok(Client::new(&apikey));
}
