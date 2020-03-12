use actix_web::{http, HttpRequest, HttpResponse};
use actix_web::http::Method;
use actix_web::web::{Data};
use handlebars::Handlebars;
use crate::templates::Curtain;

extern crate rand;
use rand::Rng;


pub async fn generate_random_string(l: i32) -> String {
    let mut text: String = "".to_string();
    let possible = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    let mut rng = rand::thread_rng();

    for _i in 0..l {
        text += &possible.chars().nth(rng.gen_range(0,62)).unwrap().to_string();
    }

    text
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
                .button("Want to login?")
                .render(data.get_ref())
                .unwrap();
            HttpResponse::Ok().body(page)
        },
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
        },
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

            let query = format!("response_type=code&client_id={}&scope={}&redirect_uri={}&state={}", 
                client_id,scope,redirect_uri,state);
            
            let uri: String = format!("{}{}","https://accounts.spotify.com/authorize?".to_string(),query);

            let res = HttpResponse::PermanentRedirect()
                .header(http::header::LOCATION, uri)
                .finish()


        },
        _ => HttpResponse::MethodNotAllowed().finish(),
    }
}