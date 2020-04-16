#![warn(missing_copy_implementations)]

// removed model in favor of rspotify's model.

mod app;
mod auth;
mod env;
mod spotify;
mod templates;
mod web;

#[macro_use]
extern crate log;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate lazy_static;

use std::io;

use actix_files as afs;
use actix_web::{middleware, web as a_web, App, HttpResponse, HttpServer};

use handlebars::Handlebars;

use crate::app::AppState;
use crate::web::*;
use std::sync::Arc;

use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

#[actix_rt::main]
async fn main() -> io::Result<()> {
    env::setup();

    // from example on https://actix.rs/docs/server/
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file(&*env::KEY_FILE, SslFiletype::PEM)
        .unwrap();
    builder
        .set_certificate_chain_file(&*env::CERT_FILE)
        .unwrap();

    let mut h = Handlebars::new();
    h.set_strict_mode(true);
    h.register_templates_directory(".hbs", "templates").unwrap();
    let data = AppState::new(Arc::new(h));
    info!("Handlebars templates registered.");

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            // logger should always be last middleware added.
            .data(data.clone())
            //.data(handlebars_ref.clone())
            .service(afs::Files::new("static/", "static/"))
            .service(a_web::resource("/is_up").to(is_up::is_up))
            .service(a_web::resource("/").to(index::index))
            .service(a_web::resource("/login").to(login::login))
            .service(a_web::resource("/callback").to(callback::callback))
            .default_service(a_web::route().to(|| HttpResponse::NotFound()))
    })
    .bind_openssl(&*env::ADDRESS, builder)?
    .run()
    .await
}
