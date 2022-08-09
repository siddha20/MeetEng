use oauth2;

pub mod creds;
pub use creds::Creds;

fn test1() {
    let test = Creds {
        client_id: "test".to_string(),
        client_secret: "test".to_string(),
        scopes: vec!["test".to_string()],
        redirect_uri: "test".to_string(),
        auth_url: "test".to_string(),
        token_url: "test".to_string(),  
    };
}