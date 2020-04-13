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
use serde_json::{Result, Value, from_str};
use crate::model;
use crate::model::{Artist, Items};

async fn extract_artists(artists: String) -> Result<Vec<Artist>> {
    let items: model::Items = from_str(artists.as_str())?;
    Ok(items.items)
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
            //artists.replace(r#"types"#, "type_str");

            let items = extract_artists(artists.clone()).await.unwrap();
            //let artists = items.artists;

            //let artists_vec = value.as_object().unwrap().get("items").unwrap();
            // let artists_map = value.as_object().unwrap();
            // let artists_vec = &artists_map.get("items").unwrap().as_array().unwrap();
            //
            // println!("artists len: {}", artists_vec.len());
            // for obj in artists_vec.iter() {
            //     println!("for {}", obj.get("name").unwrap());
            //     println!("external urls: {}", obj.get("external_urls").unwrap().get("spotify").unwrap());
            //     println!("followers: {}", obj.get("followers").unwrap().get("total").unwrap());
            //     println!("genres: {:?}", obj.get("genres").unwrap().as_array().unwrap().iter().map(|value| value.as_str().unwrap()).collect::<String>());
            // }

            //let items = &value["items"];
            //println!("artists_map: {:#?}", artists_vec);
            for artist in items {
                println!("artist: {:?}", artist);
            }

            HttpResponse::Ok().body(format!("tokens: {:?} \n\nartists:{:?}", tokens, artists))
        }
        _ => HttpResponse::MethodNotAllowed().finish(),
    }
}

