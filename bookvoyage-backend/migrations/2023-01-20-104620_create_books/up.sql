-- Your SQL goes here

CREATE TABLE books (
    id SERIAL PRIMARY KEY,
    -- todo: timestamp
    title VARCHAR NOT NULL,
    author VARCHAR NOT NULL,
    code VARCHAR NOT NULL
);

CREATE TABLE book_logs (
    id SERIAL PRIMARY KEY,
    -- todo: timestamp
    book_id INTEGER NOT NULL,
    commenter VARCHAR NOT NULL,
    comment TEXT NOT NULL,
    lat Real NOT NULL,
    lon Real NOT NULL
);