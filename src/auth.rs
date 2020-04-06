pub mod token_request;

use crate::env;
use percent_encoding::{percent_encode, NON_ALPHANUMERIC};

/// Returns the callback address.
pub fn get_callback() -> String {
    let redirect_uri = format!("https://{}/callback", *env::ADDRESS);
    percent_encode(redirect_uri.as_bytes(), NON_ALPHANUMERIC).to_string()
}