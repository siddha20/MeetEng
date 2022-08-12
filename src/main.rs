use oauth2;

mod auth;
mod api;
mod meeting;

use auth::Creds;
use api::gen_form;
use meeting::{
    Meeting,
    matcher
};


fn main() {

    let test2 = gen_form();
    println!("{}", test2.endpoint);

    let meeting = Meeting::new();
    // meeting.display();
    // println!("{:#?}", meeting);

    // matcher::perform_matching("test_data");

    let test = Creds {
        client_id: "test".to_string(),
        client_secret: "test".to_string(),
        scopes: vec!["test".to_string()],
        redirect_uri: "test".to_string(),
        auth_url: "test".to_string(),
        token_url: "test".to_string(),  
    };

    let id = oauth2::ClientId::new("1063949745433-11gul8t8tjg1nmhasssu3i9bh8e61t9s.apps.googleusercontent.com".to_string());
    let secret = Some(oauth2::ClientSecret::new("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".to_string()));
    let auth = oauth2::AuthUrl::new("https://accounts.google.com/o/oauth2/auth".to_string()).unwrap();
    let token = Some(oauth2::TokenUrl::new("https://oauth2.googleapis.com/token".to_string()).unwrap());
    let redirect = oauth2::RedirectUrl::new("http://localhost:8080".to_string()).unwrap();
    let client = oauth2::basic::BasicClient::new(id, secret, auth, token).set_redirect_uri(redirect);
    

    let (pkce_challenge, pkce_verifier) = oauth2::PkceCodeChallenge::new_random_sha256();
    let form_scope = oauth2::Scope::new("https://www.googleapis.com/auth/forms.body".to_string());
    let (auth_url, csrf_token) = client.authorize_url(oauth2::CsrfToken::new_random)
                                       .add_scope(form_scope)
                                       .set_pkce_challenge(pkce_challenge)
                                       .url();
    
    println!("User goes to {} to get code which we need.", auth_url);

    // This shouldnt work, because we dont get the code at all from the user.
    let auth_code = oauth2::AuthorizationCode::new("code from user".to_string());
    let token_result = client.exchange_code(auth_code)
                             .set_pkce_verifier(pkce_verifier)
                             .request(oauth2::reqwest::http_client);
    match token_result {
        Ok(result) => println!("Token: {:#?}", result),
        Err(err) => println!("ERROR: {}.", err),
    }

}
