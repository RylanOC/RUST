#![warn(missing_copy_implementations)]

mod templates;
mod web;
mod app;
mod spotify;

// #[macro_use] extern crate actix_web;
#[macro_use]
extern crate log;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate lazy_static;

/// Default address to use if it is not already set by an environment variable.
const DEFAULT_ADDRESS: &'static str = "127.0.0.1:8443";

/// Sets the log level as an env variable if it is not currently set.
const DEFAULT_LOG_LEVEL: &'static str = "info";

const CLIENT_ID: &'static str = "1de388fded5c43b68f60fcec9a81c956";

/// Default cert file location. Can be overridden with "CERT_FILE" env variable.
const CERT_FILE: &'static str = "cert.pem";

/// Default private key location. Can be overridden with "PRIV_KEY" env variable.
const PRIV_KEY: &'static str = "key.pem";


use std::{env, io};

use actix_files as afs;
use actix_web::{middleware, web as a_web, App, HttpResponse, HttpServer};

use handlebars::Handlebars;

use crate::web::*;
use crate::app::AppState;
use std::sync::Arc;

use rustls as tls;
use std::io::BufReader;
use rustls::internal::pemfile::{certs, rsa_private_keys};
use std::fs::File;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    let addr = env::var("BIND_TO")
        .unwrap_or(DEFAULT_ADDRESS.to_owned());

    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", DEFAULT_LOG_LEVEL);
    }

    let cert = env::var("CERT_FILE")
        .unwrap_or(CERT_FILE.to_owned());
    let priv_key = env::var("PRIV_KEY")
        .unwrap_or(PRIV_KEY.to_owned());

    env_logger::init();
    info!("Starting up.");
    info!("Current Working directory: {}", env::current_dir().unwrap().to_str().unwrap());
    info!("Address set: {}", addr);
    info!("Cert file location: {}", cert);
    info!("Private key location: {}", priv_key);
    info!("Log level set: {}", env::var("RUST_LOG").unwrap());

    // load ssl keys
    let mut tls_config = tls::ServerConfig::new(tls::NoClientAuth::new());
    let cert_file = &mut BufReader::new(File::open(cert).unwrap());
    let key_file = &mut BufReader::new(File::open(priv_key).unwrap());
    let cert_chain = certs(cert_file).unwrap();
    let mut keys = rsa_private_keys(key_file).unwrap();
    tls_config.set_single_cert(cert_chain, keys.remove(0)).unwrap();

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
    }).bind_rustls(addr, tls_config)?
        .run()
        .await
}
