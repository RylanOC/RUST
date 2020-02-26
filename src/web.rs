use actix_web::{HttpRequest, HttpResponse};
use actix_web::http::Method;

/// Check if the website is up. Responds with 200 - OK to all GET requests.
pub fn is_up(req: HttpRequest) -> HttpResponse {
    match *req.method() {
        Method::GET => HttpResponse::Ok(),
        _ => HttpResponse::MethodNotAllowed(),
    }.finish()
}
