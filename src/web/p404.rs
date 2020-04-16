use crate::app::AppState;
use crate::templates::P404;
use actix_web::http::Method;
use actix_web::web::Data;
use actix_web::{HttpRequest, HttpResponse};

/// Index page.
pub async fn p404(req: HttpRequest, app_data: Data<AppState>) -> HttpResponse {
    let hbs_reg = &app_data.template_registry;
    match *req.method() {
        Method::GET => {
            let page = P404::new(req.uri().to_string()).render(hbs_reg).unwrap();
            HttpResponse::NotFound().body(page)
        }
        _ => HttpResponse::MethodNotAllowed().finish(),
    }
}
