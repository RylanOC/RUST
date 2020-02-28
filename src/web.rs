use actix_web::{HttpRequest, HttpResponse};
use actix_web::http::Method;
use actix_web::web::{Data};
use handlebars::Handlebars;
use crate::templates::Curtain;

/// Check if the website is up. Responds with 200 - OK to all GET requests.
pub async fn is_up(req: HttpRequest, data: Data<Handlebars<'static>>) -> HttpResponse {
    match *req.method() {
        Method::GET => {
            let page = Curtain::new()
                .title("Is RUST up?")
                .subtitle("Yes it is!")
                .page_title("RUST")
                .render(data.get_ref())
                .unwrap();
            HttpResponse::Ok().body(page)
        },
        Method::POST => HttpResponse::MethodNotAllowed().finish(),
        _ => HttpResponse::NotFound().finish(),
    }
}

/// Index page.
pub async fn index(req: HttpRequest, data: Data<Handlebars<'static>>) -> HttpResponse {
    match *req.method() {
        Method::GET => {
            let page = Curtain::new()
                .page_title("RUST")
                .title("Welcome to RUST!")
                .render(data.get_ref())
                .unwrap();
            HttpResponse::Ok().body(page)
        },
        _ => HttpResponse::MethodNotAllowed().finish(),
    }
}