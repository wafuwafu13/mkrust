use crate::error::*;
use mackerel_client::*;
use std::env;

pub fn new() -> Result<Client> {
    let apikey = match env::var("MACKEREL_API_KEY") {
        Ok(apikey) => apikey,
        Err(e) => return Err(Error::EnvError(e)),
    };
    return Ok(Client::new(&apikey));
}
