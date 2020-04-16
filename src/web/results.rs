use crate::app::AppState;
use crate::auth::token_response::Tokens;
use crate::model::artists::ArtistsVec;
use crate::model::tracks::TracksVec;
use crate::spotify::{PersonalizationData, PersonalizationParams};
use crate::templates::ResultsPage;
use crate::web::TOKENS_COOKIE_NAME;
use actix_session::Session;
use actix_web::web::Data;
use actix_web::{HttpResponse, HttpRequest};
use std::process::exit;
use rspotify::senum::TimeRange;

/// Results page function. Makes calls to spotify
pub async fn results(app_data: Data<AppState>, request: HttpRequest, session: Session) -> HttpResponse {
    let query = request.uri().query().unwrap_or("medium");
    println!("head: {:#?}", request.head());
    println!("headers: {:#?}", request.headers());

    let cookies = session.get(TOKENS_COOKIE_NAME);
    if cookies.is_err() {
        return HttpResponse::InternalServerError().body(cookies.unwrap_err().to_string());
    }

    let opt = cookies.unwrap();
    if opt.is_none() {
        return HttpResponse::InternalServerError().body("cookies lost");
    }


    let mut time_range: TimeRange = TimeRange::MediumTerm;
    match query {
        "short" => time_range = TimeRange::ShortTerm,
        "medium" => time_range = TimeRange::MediumTerm,
        "long" => time_range = TimeRange::LongTerm,
        _ => time_range = TimeRange::MediumTerm
    }

    println!("query: {:?}", query);

    let tokens: Tokens = opt.unwrap();
    let artist_params = PersonalizationParams::new().limit(50).unwrap().time_range(time_range);
    let track_params = PersonalizationParams::new().limit(10).unwrap().time_range(time_range);

    let artists: ArtistsVec = PersonalizationData::Artists
        .get_data::<ArtistsVec>(&tokens, &artist_params)
        .await
        .map_err(|e| {
            error!("Could not get artist data: {}", e);
            exit(1)
        })
        .unwrap();

    let tracks: TracksVec = PersonalizationData::Tracks
        .get_data::<TracksVec>(&tokens, &track_params)
        .await
        .map_err(|e| {
            error!("Could not get track data: {}", e);
            exit(1)
        })
        .unwrap();

    let hbs_reg = &app_data.template_registry;
    let webpage = ResultsPage::new(artists, tracks).render(hbs_reg).unwrap();

    HttpResponse::Ok().body(webpage)
}
