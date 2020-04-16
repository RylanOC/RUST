use crate::app::AppState;
use crate::templates::Curtain;
use actix_web::http::Method;
use actix_web::web::Data;
use actix_web::{HttpRequest, HttpResponse};

/// Index page.
pub async fn index(req: HttpRequest, app_data: Data<AppState>) -> HttpResponse {
    let hbs_reg = &app_data.template_registry;
    match *req.method() {
        Method::GET => {
            let page = Curtain::new()
                .page_title("RUST")
                .title("Welcome to RUST!")
                .with_login_button()
                .render(hbs_reg)
                .unwrap();
            HttpResponse::Ok().body(page)
        }
        _ => HttpResponse::MethodNotAllowed().finish(),
    }
}
