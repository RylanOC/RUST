#![warn(missing_copy_implementations)]

mod templates;
mod web;

#[macro_use] extern crate actix_web;
#[macro_use] extern crate log;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate lazy_static;

const BIND_TO: &'static str = "127.0.0.1:8000";
const LOG_LEVEL: &'static str = "info";

use std::{env, io};

use log::info;

use actix_files as afs;
use actix_web::{HttpServer, App, middleware, web as a_web};

use crate::web::is_up;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", LOG_LEVEL);
    env_logger::init();
    info!("Starting up.");

    HttpServer::new(||
        App::new()
            .wrap(middleware::Logger::default()) // logger should always be last middleware added.
            .service(a_web::resource("/is_up").to(is_up))
    ).bind(BIND_TO)?
        .run()
        .await
}
