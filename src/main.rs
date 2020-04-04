#![warn(missing_copy_implementations)]

mod app;
mod spotify;
mod templates;
mod web;

#[macro_use]
extern crate log;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate lazy_static;

/// Default address to use if it is not already set by an environment variable.
const DEFAULT_ADDRESS: &'static str = "127.0.0.1:8443";
const ADDRESS_ENV_VAR: &'static str = "BIND_TO";

/// Sets the log level as an env variable if it is not currently set.
const DEFAULT_LOG_LEVEL: &'static str = "trace";
const LOG_LEVEL_ENV_VAR: &'static str = "RUST_LOG"; // this cannot change.

const CLIENT_ID: &'static str = "1de388fded5c43b68f60fcec9a81c956";

/// Default cert file location. Can be overridden with "CERT_FILE" env variable.
const CERT_FILE: &'static str = "cert.pem";
const CERT_ENV_VAR: &'static str = "CERT_FILE";

/// Default private key location. Can be overridden with "PRIV_KEY" env variable.
const PRIV_KEY: &'static str = "key.pem";
const PRIV_KEY_ENV_VAR: &'static str = "PRIV_KEY";

use std::{env, io};

use actix_files as afs;
use actix_web::{middleware, web as a_web, App, HttpResponse, HttpServer};

use handlebars::Handlebars;

use crate::app::AppState;
use crate::web::*;
use std::sync::Arc;

//use rustls as tls;
//use rustls::internal::pemfile::{certs, rsa_private_keys};
use std::fs::File;
use std::io::BufReader;
use std::process::exit;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

#[actix_rt::main]
async fn main() -> io::Result<()> {
    let addr = env::var(ADDRESS_ENV_VAR).unwrap_or(DEFAULT_ADDRESS.to_owned());

    if env::var(LOG_LEVEL_ENV_VAR).is_err() {
        env::set_var(LOG_LEVEL_ENV_VAR, DEFAULT_LOG_LEVEL);
    }

    let cert_file = env::var(CERT_ENV_VAR).unwrap_or(CERT_FILE.to_owned());
    let priv_key_file = env::var(PRIV_KEY_ENV_VAR).unwrap_or(PRIV_KEY.to_owned());

    env_logger::init();
    info!("Starting up.");
    info!(
        "Current Working directory: {}",
        env::current_dir().unwrap().to_str().unwrap()
    );
    info!("Address set: {}", addr);
    info!("Client ID: {}", CLIENT_ID);
    info!("Cert file location: {}", cert_file);
    info!("Private key location: {}", priv_key_file);
    info!("Log level set: {}", env::var(LOG_LEVEL_ENV_VAR).unwrap());

    // from example on https://actix.rs/docs/server/
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder.set_private_key_file(priv_key_file.as_str(), SslFiletype::PEM).unwrap();
    builder.set_certificate_chain_file(cert_file.as_str()).unwrap();

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
            .service(afs::Files::new("static/", "static/"))
            .service(a_web::resource("/is_up").to(is_up))
            .service(a_web::resource("/").to(index))
            .service(a_web::resource("/login").to(login))
            .service(a_web::resource("/callback").to(callback))
            .default_service(a_web::route().to(|| HttpResponse::NotFound()))
    })
    .bind_openssl("127.0.0.1:8443", builder)?
    .run()
    .await
}
