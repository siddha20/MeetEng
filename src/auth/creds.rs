pub struct Creds {
    pub client_id: String, 
    pub client_secret: String,
    pub scopes: Vec<String>,
    pub redirect_uri: String,
    pub auth_url: String,
    pub token_url: String
}