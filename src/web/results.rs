use actix_web::{HttpRequest, HttpResponse};
use actix_web::web::Data;
use crate::app::AppState;
use actix_session::Session;
use crate::web::TOKENS_COOKIE_NAME;
use crate::auth::token_response::Tokens;
use crate::spotify::PersonalizationData;
use crate::model::artists::ArtistsVec;
use std::process::exit;
use crate::model::tracks::TracksVec;

/// Results page function. Makes calls to spotif
pub async fn results(_req: HttpRequest, app_data: Data<AppState>, session: Session) -> HttpResponse {
    let cookies = session.get(TOKENS_COOKIE_NAME);
    if cookies.is_err() {
        return HttpResponse::InternalServerError().body(cookies.unwrap_err().to_string());
    }

    let opt = cookies.unwrap();
    if opt.is_none() {
        return HttpResponse::InternalServerError().body("cookies lost");
    }

    let tokens: Tokens = opt.unwrap();

    let artists: ArtistsVec = PersonalizationData::Artists
        .get_data::<ArtistsVec>(&tokens)
        .await
        .map_err(|e| {error!("Could not get artist data: {}", e); exit(1)})
        .unwrap();

    let tracks: TracksVec = PersonalizationData::Tracks
        .get_data::<TracksVec>(&tokens)
        .await
        .map_err(|e| {error!("Could not get track data: {}", e); exit(1)})
        .unwrap();

    HttpResponse::Ok().finish()
}