-- This file should undo anything in `up.sql`

-- Remove user_id from book_logs table
ALTER TABLE book_logs 
DROP COLUMN user_id;

-- Remove user_id from books table
ALTER TABLE books 
DROP COLUMN user_id;

-- Drop users table
DROP TABLE users;

-- Drop index on username
DROP INDEX idx_username;
