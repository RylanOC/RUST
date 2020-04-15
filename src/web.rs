use crate::templates::Curtain;
use crate::templates::ImageView;
use crate::barchart_maker;
use crate::histogram_maker;
use actix_web::http::Method;
use actix_web::web::Data;
use actix_web::{http, HttpRequest, HttpResponse};
use handlebars::Handlebars;

use rand::seq::IteratorRandom;

/// Generates a random string of length `l`, of any capital letters, lowercase letters,
/// and numbers.
pub async fn generate_random_string(l: usize) -> String {
    let mut buffer = vec!['\0'; l];
    let possible = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";

    // use library function instead of a loop.
    let mut rng = rand::thread_rng();
    possible.chars().choose_multiple_fill(&mut rng, &mut buffer);
    buffer.iter().collect::<String>()
}

pub async fn testchart(req: HttpRequest, data: Data<Handlebars<'static>>) -> HttpResponse {
    match *req.method() {
        Method::GET => {
            let colors = vec!["white","#1DB954","hotpink","yellow","cornflowerblue","crimson","mediumorchid"];
            let d1 = barchart_maker::BarchartDatum::new("R-B", 7.0);
            let d2 = barchart_maker::BarchartDatum::new("New Wave Classical Ska", 2.0);
            let d3 = barchart_maker::BarchartDatum::new("Sunset Groove", 18.0);
            let d4 = barchart_maker::BarchartDatum::new("Indie Pop", 4.0);
            let data_points: Vec<barchart_maker::BarchartDatum> = vec![d1,d2,d3,d4];

            barchart_maker::make_barchart(data_points,colors,"Genres", "genreTest.svg");

            let num_data = vec![0.1, 0.1, 0.2, 0.7, 0.75, 0.72, 0.9, 0.91, 0.92];

            histogram_maker::make_histogram(num_data, "num_data", "numTest.svg");

            let page = ImageView::new()
                .page_title("Test chart maker")
                .label("Genres")
                .image("genreTest.svg")
                .render(data.get_ref())
                .unwrap();
            HttpResponse::Ok().body(page)
        }
        _ => HttpResponse::NotFound().finish(),
    }
}

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
        }
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
                .with_login_button()
                .render(data.get_ref())
                .unwrap();
            HttpResponse::Ok().body(page)
        }
        _ => HttpResponse::MethodNotAllowed().finish(),
    }
}

pub async fn callback(req: HttpRequest, data: Data<Handlebars<'static>>) -> HttpResponse {
    match *req.method() {
        Method::GET => {
            let page = Curtain::new()
                .page_title("RUST")
                .title("WOOHOO!")
                .subtitle("You have successfully logged in!")
                .render(data.get_ref())
                .unwrap();
            HttpResponse::Ok().body(page)
        }
        _ => HttpResponse::MethodNotAllowed().finish(),
    }
}

/// Login should reroute to Spotify
pub async fn login(req: HttpRequest, _data: Data<Handlebars<'static>>) -> HttpResponse {
    match *req.method() {
        Method::GET => {
            let state: String = generate_random_string(16).await;
            let scope = "user-read-private%20user-read-email";
            let client_id = "1de388fded5c43b68f60fcec9a81c956";
            let redirect_uri = "http%3A%2F%2Flocalhost%3A8888%2Fcallback";

            let query = format!(
                "response_type=code&client_id={}&scope={}&redirect_uri={}&state={}",
                client_id, scope, redirect_uri, state
            );

            let uri: String = format!("https://accounts.spotify.com/authorize?{}", query);

            HttpResponse::PermanentRedirect()
                .header(http::header::LOCATION, uri)
                .finish()
        }
        _ => HttpResponse::MethodNotAllowed().finish(),
    }
}
