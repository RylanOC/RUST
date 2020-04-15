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

// async fn extract_artists(artists: String) -> Result<Value> {
//     let items: Value = from_str(artists.as_str())?;
//     Ok(items)
// }

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

            println!("artists: {:?}", artists);

            let json_value: Value = serde_json::from_str(artists.as_str()).unwrap();
            let artists_map = json_value.as_object().unwrap();
            let artist_data_vec = &artists_map.get("items").unwrap().as_array().unwrap();
            println!("artists_data_vec: {:#?}", artist_data_vec);

            let mut artist_name_vec = Vec::new();
            println!("artists len: {}", artist_data_vec.len());
            for obj in artist_data_vec.iter() {
                artist_name_vec.push(obj.get("name").unwrap().as_str().unwrap());
                println!("pushed {} into vec", obj.get("name").unwrap());

                // Extra JSON extracting examples
                // println!("external urls: {}", obj.get("external_urls").unwrap().get("spotify").unwrap());
                // println!("followers: {}", obj.get("followers").unwrap().get("total").unwrap());
                // println!("genres: {:?}", obj.get("genres").unwrap().as_array().unwrap().iter().map(|value| value.as_str().unwrap()).collect::<String>());
            }

            println!("artist_name_vec: {:#?}", artist_name_vec);
            // for artist in artist_name_vec {
            //     println!("artist: {:?}", artist);
            // }

            HttpResponse::Ok().body(format!("tokens: {:?} \n\nartists:{:?}", tokens, artists))
        }
        _ => HttpResponse::MethodNotAllowed().finish(),
    }
}
