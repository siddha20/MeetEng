use oauth2;

fn main() {
    let id = oauth2::ClientId::new("1063949745433-11gul8t8tjg1nmhasssu3i9bh8e61t9s.apps.googleusercontent.com".to_string());
    let secret = Some(oauth2::ClientSecret::new("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".to_string()));
    let auth = oauth2::AuthUrl::new("https://accounts.google.com/o/oauth2/auth".to_string()).unwrap();
    let token = Some(oauth2::TokenUrl::new("https://oauth2.googleapis.com/token".to_string()).unwrap());
    let redirect = oauth2::RedirectUrl::new("http://localhost:8888/callback".to_string()).unwrap();
    let client = oauth2::basic::BasicClient::new(id, secret, auth, token).set_redirect_uri(redirect);
    

    let (pkce_challenge, pkce_verifier) = oauth2::PkceCodeChallenge::new_random_sha256();
    let form_scope = oauth2::Scope::new("https://www.googleapis.com/auth/forms.body".to_string());
    let (auth_url, csrf_token) = client.authorize_url(oauth2::CsrfToken::new_random)
                                       .add_scope(form_scope)
                                       .set_pkce_challenge(pkce_challenge)
                                       .url();
    
    println!("User goes to {} to get code which we need.", auth_url);
    println!("csrf_token: {:#?}", csrf_token);

    // pkce_verifier used when exchanging code.
}
