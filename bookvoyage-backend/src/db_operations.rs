use crate::model::*;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_book(
    connection: &mut PgConnection,
    new_book: &NewBook,
) -> Result<Book, diesel::result::Error> {
    use crate::schema::books;

    diesel::insert_into(books::table)
        .values(new_book)
        .get_result(connection)
}

pub fn create_book_log(
    connection: &mut PgConnection,
    new_book_log: &NewBookLog,
) -> Result<BookLog, diesel::result::Error> {
    use crate::schema::book_logs;

    diesel::insert_into(book_logs::table)
        .values(new_book_log)
        .get_result(connection)
}

pub fn retrieve_book_list(
    connection: &mut PgConnection,
) -> Result<Vec<Book>, diesel::result::Error> {
    use crate::schema::books::dsl::*;

    books
        //.limit(5)
        .load::<Book>(connection)
}

pub fn retrieve_book_logs(
    connection: &mut PgConnection,
    target_book_id: i32,
) -> Result<Vec<BookLog>, diesel::result::Error> {
    use crate::schema::book_logs::dsl::*;

    book_logs
        .filter(book_id.eq(target_book_id))
        .load::<BookLog>(connection)
}

pub fn show(connection: &mut PgConnection) {
    use crate::schema::book_logs::dsl::*;
    use crate::schema::books::dsl::*;

    let book_results = books
        //.limit(5)
        .load::<Book>(connection)
        .expect("Error loading posts");

    println!("Displaying {} books", book_results.len());
    for book in book_results {
        println!("{}: \"{}\" by \"{}\"", book.id, book.title, book.author);
        println!("#BOOK {}", book.code);
        println!();

        let book_log_results = book_logs
            .filter(book_id.eq(book.id))
            //.limit(5)
            .load::<BookLog>(connection)
            .expect("Error loading posts");
        for book_log in book_log_results {
            println!(
                "\"{}\" at {:.4},{:.4}: \"{}\"",
                book_log.commenter, book_log.lat, book_log.lon, book_log.comment
            );
            println!();
        }

        println!("---------\n");
    }
}
