use crate::app::AppState;
use crate::auth::token_response::Tokens;
use crate::env;
use crate::model::artists::ArtistsVec;
use crate::model::tracks::TracksVec;
use crate::spotify::charts::ChartBuilder;
use crate::spotify::{PersonalizationData, PersonalizationParams};
use crate::templates::{Redirect, ResultsPage};
use crate::web::TOKENS_COOKIE_NAME;
use actix_session::Session;
use actix_web::http::header;
use actix_web::web::{Data, Query};
use actix_web::{HttpRequest, HttpResponse};
use rspotify::senum::TimeRange;
use std::process::exit;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ResultsQuery {
    pub time: Option<String>,
}

/// Results page function. Makes calls to spotify
pub async fn results(
    app_data: Data<AppState>,
    request: HttpRequest,
    session: Session,
) -> HttpResponse {
    let hbs_reg = &app_data.template_registry;
    let cookies = session.get(TOKENS_COOKIE_NAME);
    if cookies.is_err() {
        return HttpResponse::InternalServerError().body(cookies.unwrap_err().to_string());
    }

    let opt = cookies.unwrap();
    if opt.is_none() {
        let dest = format!("https://{}", &*env::ADDRESS);
        return HttpResponse::PermanentRedirect()
            .header(header::LOCATION, dest.clone())
            .body(Redirect::new(dest).render(&hbs_reg).unwrap());
    }

    let query_string = request.query_string();
    let query = Query::<ResultsQuery>::from_query(query_string)
        .map_err(|e| HttpResponse::BadRequest().body(e.to_string()));
    if query.is_err() {
        return query.unwrap_err();
    }
    let query = query.unwrap().into_inner();

    let time_range: TimeRange = query
        .time
        .map(|s| {
            if s == "short" {
                TimeRange::ShortTerm
            } else if s == "long" {
                TimeRange::LongTerm
            } else {
                TimeRange::MediumTerm
            }
        })
        .unwrap_or(TimeRange::MediumTerm);

    let tokens: Tokens = opt.unwrap();
    let artist_params = PersonalizationParams::new()
        .limit(50)
        .unwrap()
        .time_range(time_range);

    let track_params = PersonalizationParams::new()
        .limit(10)
        .unwrap()
        .time_range(time_range);

    let mut tracks: TracksVec = PersonalizationData::Tracks
        .get_data::<TracksVec>(&tokens, &track_params)
        .await
        .map_err(|e| {
            error!("Could not get track data: {}", e);
            exit(1)
        })
        .unwrap();

    for i in 1..6 {
        let track_params = PersonalizationParams::new()
            .offset(i * 10)
            .limit(10)
            .unwrap()
            .time_range(time_range);

        tracks.combine(
            &mut PersonalizationData::Tracks
                .get_data::<TracksVec>(&tokens, &track_params)
                .await
                .map_err(|e| {
                    error!("Could not get track data: {}", e);
                    exit(1)
                })
                .unwrap(),
        );
    }

    let artists: ArtistsVec = PersonalizationData::Artists
        .get_data::<ArtistsVec>(&tokens, &artist_params)
        .await
        .map_err(|e| {
            error!("Could not get artist data: {}", e);
            exit(1)
        })
        .unwrap();

    let mut chart_data: Vec<String> = ChartBuilder::new(artists.clone(), tracks.clone())
        .get_charts()
        .await;

    let mut charts = format!(
        "<div class=\"chart-genre\">\n{}\n</div>\n<div class=\"chart-grid\">\n",
        chart_data.pop().unwrap()
    )
    .to_string();

    for (i, chart) in chart_data.iter().enumerate() {
        let column = (i % 2) + 1;
        let row = (i / 2) + 1;
        let item = format!(
            "<div class=\"chart-item\" style=\"grid-column: {}; grid-row: {};\">\n{}\n</div>\n\n",
            column, row, chart
        );

        charts = format!("{}{}", charts, item);
    }

    charts = format!("{}</div>\n", charts);

    let webpage = ResultsPage::new(artists, tracks, charts.to_string())
        .render(hbs_reg)
        .unwrap();

    HttpResponse::Ok().body(webpage)
}
