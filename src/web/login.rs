use actix_web::{HttpRequest, HttpResponse};
use actix_web::http::{header, Method, PathAndQuery, Uri};
use rand::seq::IteratorRandom;
use std::str::FromStr;
use crate::auth;
use crate::env;
use rspotify::oauth2::{SpotifyOAuth, SpotifyClientCredentials};
use rspotify::util::get_token;
use std::process::exit;

/// Generates a random string of length `l`, of any capital letters, lowercase letters,
/// and numbers.
pub async fn generate_random_string(l: usize) -> String {
    let mut buffer = vec!['\0'; l];
    let possible = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";

    // use library function instead of a loop.
    let mut rng = rand::thread_rng();
    possible.chars().choose_multiple_fill(&mut rng, &mut buffer);
    buffer.iter().collect::<String>()
}

/// Login should reroute to Spotify
pub async fn login(req: HttpRequest) -> HttpResponse {
    match *req.method() {
        Method::GET => {
            let mut oauth = SpotifyOAuth::default()
                .redirect_uri(auth::get_callback().as_str())
                .scope("user-top-read")
                .client_id(&*env::CLIENT_ID)
                .client_secret(&*env::CLIENT_SECRET)
                .build();
            println!("{:?}", oauth);

            let token = get_token(&mut oauth)
                .await
                .unwrap_or_else(|| {
                    error!(target: "RUST::login", "Authentication Failed. Could not retrieve token");
                    exit(1);
                });

            let client_credentials = SpotifyClientCredentials::default()
                .token_info(token)
                .build();

            HttpResponse::Ok().finish()
        }
        _ => HttpResponse::MethodNotAllowed().finish(),
    }
}
