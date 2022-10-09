use mackerel_client::*;
use std::env;
use std::env::VarError;

pub fn new() -> Result<Client, VarError> {
    let apikey = match env::var("MACKEREL_API_KEY") {
        Ok(apikey) => apikey,
        Err(e) => return Err(e),
    };
    return Ok(Client::new(&apikey));
}
