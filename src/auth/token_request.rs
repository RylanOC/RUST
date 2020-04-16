//! Token request module.
//! Adapted from
//! [spotifytops](https://github.com/lperson/spotifytops/blob/main/src/lib/spotify/auth/token_request.rs)

use crate::auth::get_callback;
use crate::auth::token_response::TokenResponse;
use crate::env;
use actix_web::client::Client;
use actix_web::http::header;
use std::process::exit;

/// Represents a request to Spotify to get an auth token.
#[derive(Serialize, Debug, Clone)]
pub struct TokenRequest<'a> {
    grant_type: String,
    client_id: &'a str,
    client_secret: &'a str,
}

impl<'a> TokenRequest<'a> {
    /// Get a serialized token request.
    pub fn get_serialized_request(code: &str) -> String {
        let mut token_request = serde_urlencoded::to_string(TokenRequest {
            grant_type: "authorization_code".to_string(),
            client_id: &*env::CLIENT_ID,
            client_secret: &*env::CLIENT_SECRET,
        })
        .unwrap();

        token_request.push_str(&format!("&code={}&redirect_uri={}", code, get_callback()));

        token_request
    }

    /// Token request.
    pub async fn make_request(code: &str) -> TokenResponse {
        let serialized_token_req = TokenRequest::get_serialized_request(code);
        let client = Client::default();
        client
            .post("https://accounts.spotify.com/api/token")
            .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
            .header(header::CONTENT_LENGTH, serialized_token_req.len())
            .send_body(serialized_token_req)
            .await
            .map_err(|e| {
                error!(target: "RUST: TokenRequest::make_request", "Error getting tokens: {:?}", e);
                exit(1);
            })
            .unwrap()
            .json::<TokenResponse>()
            .await
            .unwrap()
    }
}
