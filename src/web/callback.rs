use crate::app::AppState;
use crate::auth::token_request::TokenRequest;
use crate::templates::{Redirect};
use actix_web::http::Method;
use actix_web::web::Data;
use actix_web::{HttpRequest, HttpResponse};
use std::process::exit;
use actix_session::Session;
use actix_web::http::header;
use crate::env;

/// Resource GET by spotify login response
pub async fn callback(req: HttpRequest, app_data: Data<AppState>, session: Session) -> HttpResponse {
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
                error!(target: "RUST::callback", "Token Response body was error: {:?}",
                       response.error.unwrap());
                exit(1);
            }
            let tokens = response.unwrap();

            // store the Spotify token in a cookie.
            session.set("tokens", tokens).unwrap();

            let results_page = format!("https://{}/results", &*env::ADDRESS);
            let hbs_reg = &app_data.template_registry;
            let page = Redirect::new(&results_page)
                .render(hbs_reg)
                .unwrap();
            HttpResponse::PermanentRedirect()
                .header(header::LOCATION, results_page)
                .body(page)
        }
        _ => HttpResponse::MethodNotAllowed().finish(),
    }
}
