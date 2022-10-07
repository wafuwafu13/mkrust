use mackerel_client::*;

pub async fn find_hosts(client: Client) {
    println!("{:?}", client.list_hosts().await);
}
