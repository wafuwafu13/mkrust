use mackerel_client::*;
use std::env;

pub fn new() -> Client {
    let apikey = env::var("MACKEREL_API_KEY").unwrap();
    return Client::new(&apikey);
}
