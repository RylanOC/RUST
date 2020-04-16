use crate::chart_maker::*;
use crate::templates::ImageView;
use actix_web::web::Data;
use crate::app::AppState;

use actix_web::{HttpRequest, HttpResponse};
use actix_web::http::Method;



pub async fn testchart(req: HttpRequest, app_data: Data<AppState>) -> HttpResponse {
    let hbs_reg = &app_data.template_registry;
    match *req.method() {
        Method::GET => {
            let colors = vec!["white","#1DB954","hotpink","yellow","cornflowerblue","crimson","mediumorchid"];
            let d1 = BarchartDatum::new("R-B", 7.0);
            let d2 = BarchartDatum::new("New Wave", 2.0);
            let d3 = BarchartDatum::new("Sunset Groove", 18.0);
            let d4 = BarchartDatum::new("Indie Pop", 4.0);
            let data: Vec<BarchartDatum> = vec![d1,d2,d3,d4];

            let svg = make_barchart(data,colors,"Genres");

            let page = ImageView::new()
                .page_title("Testing chart making")
                .label("Genres")
                .image(svg)
                .render(hbs_reg)
                .unwrap();
            HttpResponse::Ok().body(page)
        }
        _ => HttpResponse::NotFound().finish(),
    }
}