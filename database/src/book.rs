use diesel::insert_into;
use diesel::prelude::*;
use crate::connect::establish_connection;
use crate::error::{InsertError, QueryError, MyQueryResult, MyInsertResult};
use crate::models::{Book, NewBook};
use crate::schema::books;

pub fn get_book_by_id(id:i32) -> MyQueryResult<Book> {
    let connection = &mut establish_connection()
        .ok_or(QueryError::DatabaseNoConnect)?;
    books::table
        .find(id)
        .first::<Book>(connection)
        .map_err(|_|QueryError::NoFound)
}

pub fn get_book_by_isbn(isbn:& str) -> MyQueryResult<Book> {
    let connection = &mut establish_connection()
        .ok_or(QueryError::DatabaseNoConnect)?;
    books::table
        .filter(books::isbn.eq(isbn))
        .first::<Book>(connection)
        .map_err(|_|QueryError::NoFound)
}

pub fn get_book_by_title(title:& str) -> MyQueryResult<Vec<Book>> {
    let connection = &mut establish_connection()
        .ok_or(QueryError::DatabaseNoConnect)?;
    books::table
        .filter(books::title.eq(title))
        .load::<Book>(connection)
        .map_err(|_|QueryError::NoFound)
}


pub fn get_book_by_author(author:& str) -> MyQueryResult<Vec<Book>> {
    let connection = &mut establish_connection()
        .ok_or(QueryError::DatabaseNoConnect)?;
    books::table
        .filter(books::author.eq(author))
        .load::<Book>(connection)
        .map_err(|_|QueryError::NoFound)
}

pub fn get_book_by_category(category:& str) -> MyQueryResult<Vec<Book>> {
    let connection = &mut establish_connection()
        .ok_or(QueryError::DatabaseNoConnect)?;
    books::table
        .filter(books::category.eq(category))
        .load::<Book>(connection)
        .map_err(|_|QueryError::NoFound)
}
pub fn insert_book_by_new_book(book:&[NewBook]) ->MyInsertResult<usize> {
    let connection = &mut establish_connection()
        .ok_or(InsertError::DatabaseNoConnect)?;
    insert_into(books::table)
        .values(book)
        .execute(connection)
        .map_err(|_|InsertError::InsertConflict)
}

pub fn insert_book_by_book(book:&[Book]) ->MyInsertResult<usize> {
    let connection = &mut establish_connection()
        .ok_or(InsertError::DatabaseNoConnect)?;
    insert_into(books::table)
        .values(book)
        .execute(connection)
        .map_err(|_|InsertError::InsertConflict)
}
