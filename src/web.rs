use actix_web::{HttpRequest, HttpResponse};
use actix_web::http::Method;
use actix_web::web::Data;
use handlebars::Handlebars;

/// Check if the website is up. Responds with 200 - OK to all GET requests.
pub async fn is_up(req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().finish()
}

/// 404 page and response.
pub async fn p404(req: HttpRequest) -> HttpResponse {
    match *req.method() {
        Method::GET => HttpResponse::NotFound(),
        _ => HttpResponse::MethodNotAllowed(),
    }.finish()
}

/// Index page.
pub async fn index(req: HttpRequest, data: Data<Handlebars<'static>>) -> HttpResponse {
    match *req.method() {
        Method::GET => HttpResponse::NoContent(),
        _ => HttpResponse::MethodNotAllowed(),
    }.finish()
}