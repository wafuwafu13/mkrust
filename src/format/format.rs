#[derive(Debug)]
pub struct Host {
    pub id: String,
    pub name: String,
    pub display_name: String,
    pub status: String,
    pub memo: String,
    // pub role_fullnames: Vec<String>,
    pub is_retired: bool,
    // pub created_at: String,
    // ipaddresses: std::collections::HashMap<String, String>,
}
