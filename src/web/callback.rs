use actix_web::{HttpRequest, HttpResponse};
use actix_web::http::{Method, header, Uri};
use actix_web::web::Data;
use crate::app::AppState;
use std::str::FromStr;
use actix_web::client::Client;
use crate::auth::token_request::TokenRequest;

/// Resource GET by spotify login response
pub async fn callback(req: HttpRequest, app_data: Data<AppState>) -> HttpResponse {
    //let hbs_reg = &app_data.template_registry;
    match *req.method() {
        Method::GET => {
            let code = req
                .uri()
                .query()
                .map(|q| q.split('&').collect::<Vec<&str>>())
                .map(|v| &(v[0])[5..])
                .unwrap();

            let serialized_token_req = TokenRequest::get_serialized_request(code);
            let client = Client::default();
            let response = client
                .post("https://accounts.spotify.com/api/token")
                .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
                .header(header::CONTENT_LENGTH, serialized_token_req.len())
                .send_body(serialized_token_req)
                .await;

            HttpResponse::Ok().body(format!("{:?}", response))
        }
        _ => HttpResponse::MethodNotAllowed().finish(),
    }
}

