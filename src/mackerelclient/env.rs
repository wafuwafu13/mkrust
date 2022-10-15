use crate::error::*;
use std::env;

pub fn get_apikey() -> Result<String> {
    match env::var("MACKEREL_API_KEY") {
        Ok(apikey) => return Ok(apikey),
        Err(e) => return Err(Error::EnvError(e)),
    };
}
