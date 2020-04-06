pub mod token_request;

use crate::env;
extern crate url;
// use percent_encoding::{percent_encode, NON_ALPHANUMERIC};
use url::form_urlencoded::byte_serialize;

/// Returns the callback address.
pub fn get_callback() -> String {
    let redirect_uri = format!("https://{}/callback", *env::ADDRESS);
    redirect_uri
    //byte_serialize(redirect_uri.as_bytes()).collect()
}