use crate::app::AppState;
use crate::auth;
use crate::env;
use crate::templates::Redirect;
use actix_web::http::{header, Method, PathAndQuery, Uri};
use actix_web::web::Data;
use actix_web::{HttpRequest, HttpResponse};
use rand::seq::IteratorRandom;
use std::str::FromStr;

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
pub async fn login(req: HttpRequest, app_data: Data<AppState>) -> HttpResponse {
    match *req.method() {
        Method::GET => {
            let state: String = generate_random_string(16).await;
            let scope = "user-top-read";

            let path_and_query_str = format!(
                "/authorize?client_id={}&response_type=code&redirect_uri={}&scope={}&state={}",
                *env::CLIENT_ID,
                auth::get_callback(),
                scope,
                state
            );

            let path_and_query = PathAndQuery::from_str(path_and_query_str.as_str()).unwrap();

            let uri = Uri::builder()
                .scheme("https")
                .authority("accounts.spotify.com")
                .path_and_query(path_and_query)
                .build()
                .unwrap()
                .to_string();

            let hbs_reg = &app_data.template_registry;
            let page = Redirect::new(&uri).render(hbs_reg).unwrap();

            HttpResponse::PermanentRedirect()
                .header(header::LOCATION, uri)
                .body(page)
        }
        _ => HttpResponse::MethodNotAllowed().finish(),
    }
}
