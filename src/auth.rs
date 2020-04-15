pub mod token_request;
pub mod token_response;

use crate::env;

/// Returns the callback address.
pub fn get_callback() -> String {
    let redirect_uri = format!("https://{}/callback", *env::ADDRESS);
    redirect_uri
}