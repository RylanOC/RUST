use crate::templates::Curtain;
use actix_web::http::Method;
use actix_web::web::{Data, Bytes};
use actix_web::{http, HttpRequest, HttpResponse, client::Client};
use handlebars::Handlebars;

use rand::seq::IteratorRandom;
use actix_web::client::{ClientResponse, SendRequestError};
use actix_web::dev::{PayloadStream, Payload};
use actix_web::error::PayloadError;
use actix_web::body::Body;
use actix_web::test::read_body;
use std::io::ErrorKind;

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

/// Check if the website is up. Responds with 200 - OK to all GET requests.
pub async fn is_up(req: HttpRequest, data: Data<Handlebars<'static>>) -> HttpResponse {
    match *req.method() {
        Method::GET => {
            let page = Curtain::new()
                .title("Is RUST up?")
                .subtitle("Yes it is!")
                .page_title("RUST")
                .render(data.get_ref())
                .unwrap();
            HttpResponse::Ok().body(page)
        }
        Method::POST => HttpResponse::MethodNotAllowed().finish(),
        _ => HttpResponse::NotFound().finish(),
    }
}

/// Index page.
pub async fn index(req: HttpRequest, data: Data<Handlebars<'static>>) -> HttpResponse {
    match *req.method() {
        Method::GET => {
            let page = Curtain::new()
                .page_title("RUST")
                .title("Welcome to RUST!")
                .with_login_button()
                .render(data.get_ref())
                .unwrap();
            HttpResponse::Ok().body(page)
        }
        _ => HttpResponse::MethodNotAllowed().finish(),
    }
}

pub async fn callback(req: HttpRequest, data: Data<Handlebars<'static>>) -> HttpResponse {
    match *req.method() {
        Method::GET => {
            let page = Curtain::new()
                .page_title("RUST")
                .title("WOOHOO!")
                .subtitle("You have successfully logged in!")
                .subtitle("Getting auth tokens...")
                .render(data.get_ref())
                .unwrap();

            HttpResponse::Ok().body(page)
        }
        _ => HttpResponse::MethodNotAllowed().finish(),
    }
}

/// Login should reroute to Spotify
pub async fn login(req: HttpRequest, _data: Data<Handlebars<'static>>) -> HttpResponse {
    match *req.method() {
        Method::GET => {
            let state: String = generate_random_string(16).await;
            let scope = "user-read-private%20user-read-email";
            let client_id = "1de388fded5c43b68f60fcec9a81c956";
            let redirect_uri = "http%3A%2F%2Flocalhost%3A8888%2Fcallback";

            let query = format!(
                "response_type=code&client_id={}&scope={}&redirect_uri={}&state={}",
                client_id, scope, redirect_uri, state
            );

            let uri: String = format!("https://accounts.spotify.com/authorize?{}", query);

            HttpResponse::PermanentRedirect()
                .header(http::header::LOCATION, uri)
                .finish()
        }
        _ => HttpResponse::MethodNotAllowed().finish(),
    }
}

// pub async fn make_item_request(client: &Client, to_get: &str, timeframe: &str) {
//     let uri = format!(
//         "https://api.spotify.com/v1/me/top/{}?limit=50&time_range={}",
//         to_get, timeframe
//     );
//
//     //  Create request builder and send request
//     let response = client.get(uri)
//         .header("asdf", "wow")
//         .send()
//         .await;
//
//     response.and_then(|response| {
//         println!("Response: {:?}", response);
//         Ok(())
//     });
// }
