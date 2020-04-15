use crate::app::AppState;
use crate::model::{Artist, Items};
use crate::templates::Curtain;
use actix_web::http::Method;
use actix_web::web::Data;
use actix_web::{HttpRequest, HttpResponse};

// Constructs HTML list of recently listened to artists
pub async fn generate_artist_list(artists: Items) -> String {
    let mut table: String = "
    <tr>
        <th>Artist</th>
        <th>Popularity</th>
        <th>Genre</th>
    </tr>
    "
    .to_owned();
    for artist in artists.items {
        table.push_str("<tr>");
        table.push_str(&format!("<th>{name}</th>", name = &artist.name));
        table.push_str(&format!(
            "<th>{popularity}</th>",
            popularity = &artist.popularity
        ));
        table.push_str(&format!("<th>{genre}</th>", genre = &artist.genres[0])); //TODO: generate comma seperated list from Vec
        table.push_str("</tr>");
    }

    return table;
}

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
