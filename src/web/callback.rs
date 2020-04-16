use crate::app::AppState;
use crate::auth::token_request::TokenRequest;
use crate::auth::token_response::TokenResponse;
use crate::model;
use crate::model::{Artist, Items, Track};
use crate::spotify::PersonalizationData;
use crate::templates::Curtain;
use actix_web::client::Client;
use actix_web::http::{header, Method, Uri};
use actix_web::web::Data;
use actix_web::{HttpRequest, HttpResponse};
use serde_json::{from_str, Result, Value};
use std::process::exit;
use std::str::FromStr;

async fn get_artists(json: String) -> Vec<Artist> {
    let json_value: Value = serde_json::from_str(json.as_str()).unwrap();
    let artists_map = json_value.as_object().unwrap();
    let value_vec = &artists_map.get("items").unwrap().as_array().unwrap();

    let mut artist_vec: Vec<Artist> = Vec::new();
    for json_obj in value_vec.iter() {
        let mut genres: Vec<String> = Vec::new();
        json_obj
            .get("genres")
            .unwrap()
            .as_array()
            .unwrap()
            .iter()
            .for_each(|genre| genres.push(String::from(genre.as_str().unwrap())));
        let name = String::from(json_obj.get("name").unwrap().as_str().unwrap());
        let popularity = json_obj.get("popularity").unwrap().as_u64().unwrap();
        let uri = String::from(json_obj.get("uri").unwrap().as_str().unwrap());

        artist_vec.push(Artist {
            name,
            genres,
            popularity,
            uri,
        });
    }

    artist_vec
}

async fn get_tracks(json: String) -> Vec<Track> {
    let json_value: Value = serde_json::from_str(json.as_str()).unwrap();
    let track_map = json_value.as_object().unwrap();
    let value_vec = &track_map.get("items").unwrap().as_array().unwrap();

    let mut tracks_vec: Vec<Track> = Vec::new();
    for json_obj in value_vec.iter() {
        let name = String::from(json_obj.get("name").unwrap().as_str().unwrap());
        let uri = String::from(json_obj.get("uri").unwrap().as_str().unwrap());

        let album_json_object = json_obj.get("album").unwrap();
        let album = String::from(album_json_object.get("name").unwrap().as_str().unwrap());
        let mut artist_names: Vec<String> = Vec::new();
        let artist_json_arr = json_obj.get("artists").unwrap().as_array().unwrap();

        for artist_obj in artist_json_arr {
            artist_names.push(String::from(
                artist_obj.get("name").unwrap().as_str().unwrap(),
            ));
        }

        let popularity = json_obj.get("popularity").unwrap().as_u64().unwrap();

        tracks_vec.push(Track {
            name,
            uri,
            artist_names,
            album,
            popularity,
        });
    }

    tracks_vec
}

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
            let artists_json = PersonalizationData::Artists
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

            let artist_vec = get_artists(artists_json.clone()).await;

            let tracks_json = PersonalizationData::Tracks
                .make_req(&tokens)
                .send()
                .await
                .map_err(|e| {
                    error!(target: "RUST::callback", "Error getting track data: {:?}", e);
                    exit(1);
                })
                .unwrap()
                .body()
                .await
                .unwrap()
                .iter()
                .map(|byte| *byte as char)
                .collect::<String>();

            let tracks_vec = get_tracks(tracks_json.clone()).await;
            println!("tracks_vec: {:#?}", tracks_vec);

            let hbs_reg = &app_data.template_registry;
            //let test_vec = vec!["a".to_string(), "b".to_string(), "c".to_string()];
            let page = Curtain::new()
                .page_title("RUST")
                .title("Artist List")
                //.artist_list(test_vec)
                .artist_list(artist_vec)
                .render(hbs_reg)
                .unwrap();
            HttpResponse::Ok().body(page)
        }
        _ => HttpResponse::MethodNotAllowed().finish(),
    }
}
