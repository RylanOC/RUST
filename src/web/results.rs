use crate::app::AppState;
use crate::auth::token_response::Tokens;
use crate::model::artists::ArtistsVec;
use crate::model::tracks::TracksVec;
use crate::spotify::{PersonalizationData, PersonalizationParams};
use crate::templates::ResultsPage;
use crate::web::TOKENS_COOKIE_NAME;
use actix_session::Session;
use actix_web::web::{Data, Query};
use actix_web::{HttpResponse, HttpRequest};
use std::process::exit;
use rspotify::senum::TimeRange;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ResultsQuery {
    pub time: Option<String>
}

/// Results page function. Makes calls to spotify
pub async fn results(app_data: Data<AppState>, request: HttpRequest, session: Session) -> HttpResponse {
    let cookies = session.get(TOKENS_COOKIE_NAME);
    if cookies.is_err() {
        return HttpResponse::InternalServerError().body(cookies.unwrap_err().to_string());
    }

    let opt = cookies.unwrap();
    if opt.is_none() {
        return HttpResponse::InternalServerError().body("cookies lost");
    }

    let query_string = request.query_string();
    let query = Query::<ResultsQuery>::from_query(query_string)
        .map_err(|e| HttpResponse::BadRequest().body(e.to_string()));
    if query.is_err() {
        return query.unwrap_err();
    }
    let query = query.unwrap().into_inner();

    let time_range: TimeRange = query.time.map(|s| {
        if s == "short" {TimeRange::ShortTerm}
        else if s == "long" {TimeRange::LongTerm }
        else {TimeRange::MediumTerm}
    }).unwrap_or(TimeRange::MediumTerm);

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
