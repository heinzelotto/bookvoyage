use actix_web::{
    get, middleware::Logger, post, web, App, HttpResponse, HttpServer, Responder, Result,
};
use serde_derive::{Deserialize, Serialize};

mod db_operations;
mod model;
mod schema;

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
    lat: f32,
    lon: f32,
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
    let post = dbg!(data.into_inner());
    let mut connection = db_operations::establish_connection();

    let new_book = model::NewBook {
        title: post.title,
        author: post.author,
        code: post.code,
    };
    let created_book = db_operations::create_book(&mut connection, &new_book)
        .expect("New book could not be created.");

    let first_log = model::NewBookLog {
        book_id: created_book.id,
        commenter: String::from("anonymous"),
        comment: post.review,
        lat: post.lat,
        lon: post.lon,
    };
    let created_first_log = db_operations::create_book_log(&mut connection, &first_log)
        .expect("First logging could not be created.");

    db_operations::show(&mut connection);

    Ok(dbg!(web::Json(SendResponse {
        response: format!(
            "Book inserted into db with ID {}. Book log inserted into db with ID {}",
            created_book.id, created_first_log.id
        )
    })))
}

#[get("/book_list")]
async fn book_list() -> Result<impl Responder> {
    let mut connection = db_operations::establish_connection();

    let l =
        db_operations::retrieve_book_list(&mut connection).expect("Couldn't retreive book list");

    Ok(dbg!(web::Json(l)))
}

#[get("/book_list/by_code/{book_code}")]
async fn book_list_by_code(path: web::Path<String>) -> Result<impl Responder> {
    let book_code = path.into_inner();

    let mut connection = db_operations::establish_connection();

    let l = db_operations::retrieve_books_by_code(&mut connection, &book_code)
        .expect("Couldn't retreive book for code {book_code}");

    Ok(dbg!(web::Json(l)))
}

#[get("/book_logs/by_id/{book_id}")]
async fn book_logs_by_id(path: web::Path<i32>) -> Result<impl Responder> {
    let book_id = path.into_inner();

    let mut connection = db_operations::establish_connection();

    let l = db_operations::retrieve_book_logs_by_id(&mut connection, book_id)
        .expect("Couldn't retreive book logs for id {book_id}");

    Ok(dbg!(web::Json(l)))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(hello)
            .service(code)
            .service(send)
            .service(book_list)
            .service(book_list_by_code)
            .service(book_logs_by_id)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
