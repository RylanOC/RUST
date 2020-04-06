use actix_web::{HttpRequest, HttpResponse};
use actix_web::http::{Method, header, Uri};
use actix_web::web::Data;
use crate::app::AppState;
use std::str::FromStr;
use actix_web::client::Client;
use crate::auth::token_request::TokenRequest;
use crate::auth::token_response::TokenResponse;
use std::process::exit;
use crate::spotify::PersonalizationData;

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

            let response = TokenRequest::make_request(code).await;
            if response.is_error() {
                error!(target: "RUST::callback", "Response body was error: {:?}",
                       response.error.unwrap());
                exit(1);
            }
            let tokens = response.unwrap();
            let artists = PersonalizationData::Artists
                .make_req(&tokens)
                .send()
                .await
                .map_err(|e| {
                    error!(target: "RUST::callback", "Error getting artist data: {:?}", e);
                    exit(1);
                })
                .unwrap()
                .body()
                .await
                .unwrap()
                .iter()
                .map(|byte| *byte as char)
                .collect::<String>();

            HttpResponse::Ok().body(format!("tokens: {:?} \n\nartists:{}", tokens, artists))
        }
        _ => HttpResponse::MethodNotAllowed().finish(),
    }
}

