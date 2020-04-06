use actix_web::{HttpRequest, HttpResponse};
use actix_web::http::{Method, PathAndQuery, Uri, header};
use actix_web::web::Data;
use crate::app::AppState;
use crate::templates::Curtain;
use regex::Regex;

use actix_web::client::Client;
use crate::auth::token_request::TokenRequest;

lazy_static! {
    static ref QUERY_REGEX: Regex = Regex::new("code=(.+)").unwrap();
}

/// Resource GET by spotify login response
pub async fn callback(req: HttpRequest, app_data: Data<AppState>) -> HttpResponse {
    let hbs_reg = &app_data.template_registry;
    match *req.method() {
        Method::GET => {
            let query = req.uri().query();
            let re: &Regex = &QUERY_REGEX;
            let code = query
                .and_then(|q| re.captures(q))
                .and_then(|caps| caps.get(0))
                .map(|re_match| re_match.as_str())
                .unwrap();

            let serialized_token_req = TokenRequest::get_serialized_request(code);
            let client = Client::default();
            let response = client
                .post("https://accounts.spotify.com/api/token")
                .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
                .header(header::CONTENT_LENGTH, serialized_token_req.len())
                .send_body(serialized_token_req)
                .await;

            // let page = Curtain::new()
            //     .page_title("RUST")
            //     .title("WOOHOO!")
            //     .subtitle(tracks.map_or("unable to get".to_owned(), |s| s))
            //     .render(hbs_reg)
            //     .unwrap();

            HttpResponse::Ok().body(format!("{:?}", response))
        }
        _ => HttpResponse::MethodNotAllowed().finish(),
    }
}

