#![warn(missing_copy_implementations)]

mod app;
mod spotify;
mod templates;
mod web;

// #[macro_use] extern crate actix_web;
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
const DEFAULT_LOG_LEVEL: &'static str = "info";
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

use rustls as tls;
use rustls::internal::pemfile::{certs, rsa_private_keys};
use std::fs::File;
use std::io::BufReader;
use std::process::exit;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    let addr = env::var(ADDRESS_ENV_VAR).unwrap_or(DEFAULT_ADDRESS.to_owned());

    if env::var(LOG_LEVEL_ENV_VAR).is_err() {
        env::set_var(LOG_LEVEL_ENV_VAR, DEFAULT_LOG_LEVEL);
    }

    let cert = env::var(CERT_ENV_VAR).unwrap_or(CERT_FILE.to_owned());
    let priv_key = env::var(PRIV_KEY_ENV_VAR).unwrap_or(PRIV_KEY.to_owned());

    env_logger::init();
    info!("Starting up.");
    info!(
        "Current Working directory: {}",
        env::current_dir().unwrap().to_str().unwrap()
    );
    info!("Address set: {}", addr);
    info!("Cert file location: {}", cert);
    info!("Client ID: {}", CLIENT_ID);
    info!("Private key location: {}", priv_key);
    info!("Log level set: {}", env::var(LOG_LEVEL_ENV_VAR).unwrap());

    // load ssl keys
    let mut tls_config = tls::ServerConfig::new(tls::NoClientAuth::new());

    let mut cert_file = File::open(cert.clone())
        .map_err(|e| {
            error!("Could not open cert file at {}", cert);
            exit(e.raw_os_error().unwrap_or(1))
        })
        .map(BufReader::new)
        .unwrap();

    let mut key_file = File::open(priv_key.clone())
        .map_err(|e| {
            error!("Could not read private key at {}", priv_key);
            exit(e.raw_os_error().unwrap_or(1))
        })
        .map(BufReader::new)
        .unwrap();

    let cert_chain = certs(&mut cert_file).unwrap();
    let mut keys = rsa_private_keys(&mut key_file).unwrap();
    tls_config
        .set_single_cert(cert_chain, keys.remove(0))
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
            .service(afs::Files::new("static/", "static/"))
            .service(a_web::resource("/is_up").to(is_up))
            .service(a_web::resource("/").to(index))
            .service(a_web::resource("/login").to(login))
            .service(a_web::resource("/callback").to(callback))
            .default_service(a_web::route().to(|| HttpResponse::NotFound()))
    })
    .bind_rustls(addr, tls_config)?
    .run()
    .await
}
