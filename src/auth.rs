pub mod token_request;

use crate::env;

/// Returns the callback address.
pub fn get_callback() -> String {
    let redirect_uri = format!("https://{}/callback", *env::ADDRESS);
    redirect_uri
}