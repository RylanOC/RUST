use crate::app::AppState;
use crate::auth::token_request::TokenRequest;
use crate::spotify::PersonalizationData;
use crate::templates::Curtain;
use actix_web::http::Method;
use actix_web::web::Data;
use actix_web::{HttpRequest, HttpResponse};
use std::process::exit;
use crate::model::artists::ArtistsVec;
use actix_session::Session;

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
                .json::<ArtistsVec>()
                .await
                .unwrap();

            println!("{:#?}", artists);

            let hbs_reg = &app_data.template_registry;
            let page = Curtain::new()
                .page_title("RUST")
                .title("Artist List")
                .render(hbs_reg)
                .unwrap();
            HttpResponse::Ok().body(page)
        }
        _ => HttpResponse::MethodNotAllowed().finish(),
    }
}
