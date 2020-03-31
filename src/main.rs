#![warn(missing_copy_implementations)]

mod templates;
mod web;

// #[macro_use] extern crate actix_web;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;
// #[macro_use] extern crate lazy_static;

const BIND_TO: &'static str = "127.0.0.1:8888";
const LOG_LEVEL: &'static str = "info";

use std::{env, io};

use actix_files as afs;
use actix_web::{middleware, web as a_web, App, HttpResponse, HttpServer, client::Client};

use handlebars::Handlebars;

use crate::web::*;

struct AppState {
    client: Client,
    client_id: String,
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", LOG_LEVEL);
    env_logger::init();
    info!("Starting up.");

    let mut h = Handlebars::new();
    h.set_strict_mode(true);
    h.register_templates_directory(".hbs", "templates").unwrap();
    let handlebars_ref = a_web::Data::new(h);
    info!("Handlebars templates registered.");

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            // logger should always be last middleware added.
            .app_data(handlebars_ref.clone())
            .data(AppState {
                client: Client::default(),
                client_id: String::from("1de388fded5c43b68f60fcec9a81c956"), // maybe figure out a way to not hard code this
            })
            .service(afs::Files::new("static/", "static/"))
            .service(a_web::resource("/is_up").to(is_up))
            .service(a_web::resource("/").to(index))
            .service(a_web::resource("/login").to(login))
            .service(a_web::resource("/callback").to(callback))
            .default_service(a_web::route().to(|| HttpResponse::NotFound()))
    }).bind(BIND_TO)?
        .run()
        .await
}
