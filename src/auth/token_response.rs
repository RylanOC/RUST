//! Token response type. Inspired by
//! [spotifytops](https://github.com/lperson/spotifytops/blob/main/src/lib/spotify/auth/token_response.rs).
#[derive(Deserialize, Debug, Clone)]
pub struct AuthenticationError {
    pub error: String,
    pub error_description: String,
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct TokenResponse {
    pub error: Option<AuthenticationError>,
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
    pub token_type: Option<String>,
    pub expires_in: Option<u32>,
    pub scope: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Tokens {
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,
    pub expires_in: u32,
    pub scope: String,
}

impl TokenResponse {
    // old piece of code from spotify tops
    /* pub fn new_error(error: String, error_description: String) -> TokenResponse {
       TokenResponse {
           error: Some(AuthenticationError {
               error,
               error_description,
           }),
           access_token: None,
           refresh_token: None,
           token_type: None,
           expires_in: None,
           scope: None,
       }
    } */

    #[inline]
    pub fn is_error(&self) -> bool {
        self.error.is_some()
    }

    /// If this response isn't an error, than unwrap the important bits.
    /// Panics if this is an error.
    pub fn unwrap(self) -> Tokens {
        if self.is_error() {
            panic!("Panic on unwrapping TokenResponse")
        };
        Tokens {
            access_token: self.access_token.unwrap(),
            refresh_token: self.refresh_token.unwrap(),
            token_type: self.token_type.unwrap(),
            expires_in: self.expires_in.unwrap(),
            scope: self.scope.unwrap(),
        }
    }
}
