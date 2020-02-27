#![warn(missing_copy_implementations)]

mod templates;
mod web;

#[macro_use] extern crate actix_web;
#[macro_use] extern crate log;
#[macro_use] extern crate serde_derive;
//#[macro_use] extern crate lazy_static;

const BIND_TO: &'static str = "127.0.0.1:8000";
const LOG_LEVEL: &'static str = "info";

use std::{env, io};

use actix_files as afs;
use actix_web::{HttpServer, App, middleware, web as a_web};

use handlebars::Handlebars;

use crate::web::*;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", LOG_LEVEL);
    env_logger::init();
    info!("Starting up.");

    let mut h = Handlebars::new();
    h.set_strict_mode(true);
    h.register_templates_directory(".hbs", "static").unwrap();
    let handlebars_ref = a_web::Data::new(h);
    info!("Handlebars templates registered.");

    HttpServer::new(move ||
        App::new()
            .wrap(middleware::Logger::default())
                // logger should always be last middleware added.
            .data(handlebars_ref.clone())
            .service(a_web::resource("/is_up").get().to(is_up))
            // `.get()` is a Guard -- requires GET request otherwise return 405-Method Not Allowed
            .default_service(a_web::resource("").to(p404))
    ).bind(BIND_TO)?
        .run()
        .await
}
