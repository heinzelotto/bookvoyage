use actix_web::{
    dev::ServiceRequest, get, middleware::Logger, post, web, App, Error, HttpMessage, HttpResponse,
    HttpServer, Responder, Result,
};
use actix_web_httpauth::extractors::AuthenticationError;
use actix_web_httpauth::{
    extractors::bearer, extractors::bearer::BearerAuth, middleware::HttpAuthentication,
};
use bcrypt::{hash, verify};
use chrono::{Duration, Utc};
use diesel::ExpressionMethods;
use diesel::{query_dsl::methods::FilterDsl, PgConnection, Queryable, RunQueryDsl};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use serde_json::json;

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

#[derive(Deserialize, Debug)]
struct AddLogPost {
    code: String,
    review: String,
    lat: f32,
    lon: f32,
}

// User struct for database
#[derive(Queryable, Serialize, Deserialize)]
struct User {
    id: i32,
    username: String,
    password_hash: String,
}

// New user registration
#[derive(Deserialize)]
struct NewUser {
    username: String,
    password: String,
}

// Login credentials
#[derive(Deserialize)]
struct Credentials {
    username: String,
    password: String,
}

// JWT claims
#[derive(Debug, Serialize, Deserialize, Clone)]
struct Claims {
    sub: String,
    exp: i64,
    iat: i64,
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
async fn send(data: web::Json<SendPost>, claims: web::ReqData<Claims>) -> Result<impl Responder> {
    let post = dbg!(data.into_inner());
    let mut connection = db_operations::establish_connection();

    let new_book = model::NewBook {
        title: &post.title,
        author: &post.author,
        code: &post.code,
        user_id: claims.sub.parse::<i32>().ok(),
    };
    let created_book = db_operations::create_book(&mut connection, &new_book)
        .expect("New book could not be created.");

    let first_log = model::NewBookLog {
        book_id: created_book.id,
        commenter: &String::from("anonymous"),
        comment: &post.review,
        lat: post.lat,
        lon: post.lon,
        user_id: claims.sub.parse::<i32>().ok(),
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

#[post("/add_log")]
async fn add_log(
    data: web::Json<AddLogPost>,
    claims: web::ReqData<Claims>,
) -> Result<impl Responder> {
    let post = dbg!(data.into_inner());
    let mut connection = db_operations::establish_connection();

    dbg!(&post);

    let book_id = db_operations::retrieve_books_by_code(&mut connection, &post.code)
        .expect("Couldn't retreive book for code {book_code}")
        .first()
        .unwrap()
        .id;

    let new_book_log = model::NewBookLog {
        book_id: book_id,
        commenter: &String::from("anonymous"),
        comment: &post.review,
        lat: post.lat,
        lon: post.lon,
        user_id: claims.sub.parse::<i32>().ok(),
    };
    let created_log = db_operations::create_book_log(&mut connection, &new_book_log)
        .expect("New log could not be created.");

    db_operations::show(&mut connection);

    Ok(dbg!(web::Json(SendResponse {
        response: format!(
            "Book log inserted into db with ID {} (for book with ID {})",
            created_log.id, book_id
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

    // TODO: can we reuse the connection?
    let mut connection = db_operations::establish_connection();

    let l = db_operations::retrieve_book_logs_by_id(&mut connection, book_id)
        .expect("Couldn't retreive book logs for id {book_id}");

    Ok(dbg!(web::Json(l)))
}

#[get("/users")]
async fn users() -> Result<impl Responder> {
    let mut connection = db_operations::establish_connection();

    let l = db_operations::retrieve_user_list(&mut connection).expect("Couldn't retreive users");

    Ok(dbg!(web::Json(l)))
}

// Registration handler
#[post("/register")]
async fn register(user: web::Json<NewUser>) -> Result<impl Responder> {
    let mut connection = db_operations::establish_connection();

    let hash = hash(&user.password, 7).unwrap();
    let new_user = model::NewUser {
        username: &user.username,
        password_hash: &hash,
    };

    match db_operations::create_user(&mut connection, &new_user) {
        Ok(created_user) => Ok(HttpResponse::Ok().json(dbg!((created_user)))),
        Err(_) => Ok(HttpResponse::Ok().json("Could not create user")),
    }
}

// Login handler
#[post("/login")]
async fn login(creds: web::Json<Credentials>) -> impl Responder {
    let mut connection = db_operations::establish_connection();

    let user_result = db_operations::retrieve_user_by_username(&mut connection, &creds.username);

    match user_result {
        Ok(user) => {
            if verify(&creds.password, &user.password_hash).unwrap() {
                let expiration = Utc::now()
                    .checked_add_signed(Duration::hours(2))
                    .expect("valid timestamp")
                    .timestamp();

                let claims = Claims {
                    sub: user.username,
                    exp: expiration,
                    iat: Utc::now().timestamp(),
                };

                let token = encode(
                    &Header::default(),
                    &claims,
                    &EncodingKey::from_secret("secret".as_ref()),
                )
                .unwrap();

                HttpResponse::Ok().json(json!({"token": token}))
            } else {
                HttpResponse::Unauthorized().json("Invalid credentials")
            }
        }
        Err(_) => HttpResponse::Unauthorized().json("User not found"),
    }
}

// Middleware for token validation
async fn validate_token(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    dbg!(&credentials);
    dbg!(&credentials.token());
    let token_string = credentials.token();
    // TODO: get from .env, Hmac sha256 it
    let key = b"secret";
    match decode::<Claims>(
        token_string,
        &DecodingKey::from_secret(key),
        &Validation::default(),
    ) {
        Ok(token_data) => {
            req.extensions_mut().insert(token_data.claims);
            Ok(req)
        }
        Err(_) => {
            let config = req
                .app_data::<bearer::Config>()
                .cloned()
                .unwrap_or_default()
                .scope("");

            Err((AuthenticationError::from(config).into(), req))
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    HttpServer::new(move || {
        let bearer_middleware = HttpAuthentication::bearer(validate_token);

        App::new()
            .wrap(Logger::default())
            // unauthenticated routes
            .service(hello)
            .service(register)
            .service(code)
            .service(login)
            .service(users)
            // authenticated routes
            .service(
                web::scope("")
                    .wrap(bearer_middleware)
                    .service(send)
                    .service(book_list)
                    .service(book_list_by_code)
                    .service(book_logs_by_id)
                    .service(add_log),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
