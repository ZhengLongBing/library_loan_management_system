use diesel::insert_into;
use diesel::prelude::*;
use crate::connect::establish_connection;
use crate::error::{InsertError, QueryError, MyQueryResult, MyInsertResult};
use crate::models::{BorrowRecord, NewBorrowRecord};
use crate::schema::{books, borrow_records};



pub fn get_borrow_record_by_id(id:i32) -> MyQueryResult<BorrowRecord> {
    let connection = &mut establish_connection()
        .ok_or(QueryError::DatabaseNoConnect)?;
    borrow_records::table
        .find(id)
        .first::<BorrowRecord>(connection)
        .map_err(QueryError::NoFound)
}

pub fn get_borrow_record_by_account_id(account_id:i32) -> MyQueryResult<Vec<BorrowRecord>> {
    let connection = &mut establish_connection()
        .ok_or(QueryError::DatabaseNoConnect)?;
    borrow_records::table
        .filter(borrow_records::account_id.eq(account_id))
        .load::<BorrowRecord>(connection)
        .map_err(QueryError::NoFound)
}

pub fn get_borrow_record_by_book_id(book_id:i32) -> MyQueryResult<Vec<BorrowRecord>> {
    let connection = &mut establish_connection()
        .ok_or(QueryError::DatabaseNoConnect)?;
    borrow_records::table
        .filter(borrow_records::book_id.eq(book_id))
        .load::<BorrowRecord>(connection)
        .map_err(QueryError::NoFound)
}

pub fn get_borrow_record_by_book_isbn(book_isbn:& str) -> MyQueryResult<Vec<BorrowRecord>> {
    let connection = &mut establish_connection()
        .ok_or(QueryError::DatabaseNoConnect)?;
    borrow_records::table
        .inner_join(books::table)
        .filter(books::isbn.eq(book_isbn))
        .select((
            borrow_records::id,
            borrow_records::account_id,
            borrow_records::book_id,
            borrow_records::borrow_date,
            borrow_records::as_of_date,
        ))
        .load::<BorrowRecord>(connection)
        .map_err(QueryError::NoFound)
}

pub fn get_borrow_record_by_book_title(book_title:& str) -> MyQueryResult<Vec<BorrowRecord>> {
    let connection = &mut establish_connection()
        .ok_or(QueryError::DatabaseNoConnect)?;
    borrow_records::table
        .inner_join(books::table)
        .filter(books::title.eq(book_title))
        .select((
            borrow_records::id,
            borrow_records::account_id,
            borrow_records::book_id,
            borrow_records::borrow_date,
            borrow_records::as_of_date,
        ))
        .load::<BorrowRecord>(connection)
        .map_err(QueryError::NoFound)
}
pub fn insert_borrow_record_by_new_borrow_record(borrow_record:&[NewBorrowRecord]) ->MyInsertResult<usize> {
    let connection = &mut establish_connection()
        .ok_or(InsertError::DatabaseNoConnect)?;
    insert_into(borrow_records::table)
        .values(borrow_record)
        .execute(connection)
        .map_err(InsertError::InsertConflict)
}