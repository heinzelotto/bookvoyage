-- Your SQL goes here

-- Create users table
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(50) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Add user_id to books table
ALTER TABLE books 
ADD COLUMN user_id INTEGER REFERENCES users(id);

-- Add user_id to book_logs table
ALTER TABLE book_logs 
ADD COLUMN user_id INTEGER REFERENCES users(id);

-- Create index on username for faster lookups
CREATE INDEX idx_username ON users(username);