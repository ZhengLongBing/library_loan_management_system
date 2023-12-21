-- This file should undo anything in `up.sql`
DROP VIEW   book_counts;
DROP VIEW   borrow_counts;
DROP VIEW   remaining_books;
DROP VIEW   user_borrow_details;
DROP VIEW   user_return_details;
DROP VIEW books_display;

DROP TABLE borrow_records;
DROP TABLE borrow_and_return_records;
DROP TABLE accounts;
DROP TABLE books;
