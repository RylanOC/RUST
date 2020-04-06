//! Token request module.
//! Adapted from
//! [spotifytops](https://github.com/lperson/spotifytops/blob/main/src/lib/spotify/auth/token_request.rs)

use crate::env;
use crate::auth::get_callback;

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
        }).unwrap();

        token_request.push_str(&format!("&code={}&redirect_uri={}", code, get_callback()));

        token_request
    }
}