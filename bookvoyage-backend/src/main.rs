use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result};
use rand::prelude::random;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
struct BookCode {
    code_string: String,
}

// TODO: next step make docker container as in https://genekuo.medium.com/creating-a-rest-api-in-rust-with-persistence-rust-rocket-and-diesel-a4117d400104

// ambiguous according to pwgen (Theodore Ts'o): "B8G6I1l0OQDS5Z2"
// const CHARACTERS_UNAMBIGUOUS: &str = "3479ACEFHJKLMNPRTUVWXY";
// we use exactly 16 letters to get a nice coding
const CHARSET: [char; 16] = [
    '3', '4', '9', 'C', 'F', 'H', 'J', 'K', 'L', 'M', 'N', 'P', 'R', 'T', 'W', 'X',
];

impl BookCode {
    fn new() -> Self {
        // 32-bit codeword
        let r = rand::random::<[u8; 8]>();
        let code_string = r
            .iter()
            .map(|i| CHARSET[(*i as usize) % CHARSET.len()])
            .collect::<String>();

        Self { code_string }
    }
}

#[derive(Serialize, Debug)]
struct SendResponse {
    response: String,
}

#[derive(Deserialize, Debug)]
struct SendPost {
    title: String,
    author: String,
    review: String,
    code: String,
    coords: String,
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/code")]
async fn code() -> Result<impl Responder> {
    let code = BookCode::new();

    Ok(dbg!(web::Json(code)))
}

#[post("/send")]
async fn send(data: web::Json<SendPost>) -> Result<impl Responder> {
    dbg!(
        &data.title,
        &data.author,
        &data.review,
        &data.code,
        &data.coords
    );
    Ok(dbg!(web::Json(SendResponse {
        response: String::from("This is just a mock for now. Book will be added to database soon!")
    })))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(hello).service(code).service(send))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
