use actix_web::{HttpRequest, HttpResponse};
use actix_web::web::Data;
use crate::app::AppState;
use actix_web::http::Method;
use crate::templates::Curtain;

/// Check if the website is up. Responds with 200 - OK to all GET requests.
pub async fn is_up(req: HttpRequest, app_data: Data<AppState>) -> HttpResponse {
    let hbs_reg = &app_data.template_registry;
    match *req.method() {
        Method::GET => {
            let page = Curtain::new()
                .title("Is RUST up?")
                .subtitle("Yes it is!")
                .page_title("RUST")
                .render(hbs_reg)
                .unwrap();
            HttpResponse::Ok().body(page)
        }
        _ => HttpResponse::MethodNotAllowed().finish(),
    }
}