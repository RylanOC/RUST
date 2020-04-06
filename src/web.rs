use crate::templates::Curtain;
use actix_web::http::{Method, PathAndQuery, Uri};
use actix_web::web::Data;
use actix_web::{http, HttpRequest, HttpResponse};

use crate::app::AppState;
use crate::spotify::PersonalizationData;
use actix_web::client::ClientBuilder;
use rand::seq::IteratorRandom;

use crate::env;
use percent_encoding::{percent_encode, NON_ALPHANUMERIC};
use regex::Regex;
use std::str::FromStr;

lazy_static! {
    static ref QUERY_REGEX: Regex = Regex::new("code=(.+)").unwrap();
}

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
pub async fn is_up(req: HttpRequest, app_data: Data<AppState>) -> HttpResponse {
    let hbs_reg = &app_data.template_registry;
    match *req.method() {
        Method::GET => {
            let page = Curtain::new()
                .title("Is RUST up?")
                .subtitle("Yes it is!")
                .page_title("RUST")
                .render(hbs_reg)
                .unwrap();
            HttpResponse::Ok().body(page)
        }
        _ => HttpResponse::MethodNotAllowed().finish(),
    }
}

/// Index page.
pub async fn index(req: HttpRequest, app_data: Data<AppState>) -> HttpResponse {
    let hbs_reg = &app_data.template_registry;
    match *req.method() {
        Method::GET => {
            let page = Curtain::new()
                .page_title("RUST")
                .title("Welcome to RUST!")
                .with_login_button()
                .render(hbs_reg)
                .unwrap();
            HttpResponse::Ok().body(page)
        }
        _ => HttpResponse::MethodNotAllowed().finish(),
    }
}

pub async fn callback(req: HttpRequest, app_data: Data<AppState>) -> HttpResponse {
    let hbs_reg = &app_data.template_registry;
    match *req.method() {
        Method::GET => {
            let query = req.uri().query();
            let artists: Option<String> = None;
            let mut tracks: Option<String> = None;
            let re: &Regex = &QUERY_REGEX;
            let code = query
                .and_then(|q| re.captures(q))
                .and_then(|caps| caps.get(0))
                .map(|re_match| re_match.as_str());
            if code.is_some() {
                let client = ClientBuilder::new()
                    .header(http::header::AUTHORIZATION, code.unwrap())
                    .finish();
                let res = client
                    .get(PersonalizationData::Tracks.get_endpoint().to_string())
                    .send()
                    .await;
                tracks = Some(
                    res.unwrap()
                        .body()
                        .await
                        .unwrap()
                        .iter()
                        .map(|b| *b as char)
                        .collect::<String>(),
                );
            }

            let page = Curtain::new()
                .page_title("RUST")
                .title("WOOHOO!")
                .subtitle(tracks.map_or("unable to get".to_owned(), |s| s))
                .render(hbs_reg)
                .unwrap();

            HttpResponse::Ok().body(page)
        }
        _ => HttpResponse::MethodNotAllowed().finish(),
    }
}

/// Login should reroute to Spotify
pub async fn login(req: HttpRequest) -> HttpResponse {
    match *req.method() {
        Method::GET => {
            let state: String = generate_random_string(16).await;
            let scope = "user-top-read";

            let redirect_uri = format!("https://{}/callback", *env::ADDRESS);

            let path_and_query_str = format!(
                "/authorize?response_type=code&client_id={}&scope={}&redirect_uri={}&state={}",
                *env::CLIENT_ID,
                scope,
                percent_encode(redirect_uri.as_bytes(), NON_ALPHANUMERIC).to_string(),
                state
            );

            debug!("Callback path_and_query: {}", path_and_query_str);

            let path_and_query = PathAndQuery::from_str(path_and_query_str.as_str()).unwrap();

            let uri = Uri::builder()
                .scheme("https")
                .authority("accounts.spotify.com")
                .path_and_query(path_and_query)
                .build()
                .unwrap();

            debug!("Callback uri: {}", uri);

            HttpResponse::PermanentRedirect()
                .header(http::header::LOCATION, uri.to_string())
                .finish()
        }
        _ => HttpResponse::MethodNotAllowed().finish(),
    }
}
