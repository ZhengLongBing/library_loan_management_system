use diesel::insert_into;
use diesel::prelude::*;
use crate::connect::establish_connection;
use crate::error::{InsertError, QueryError, MyQueryResult, MyInsertResult};
use crate::models::{BorrowAndReturnRecord, NewBorrowAndReturnRecord};
use crate::schema::{books, borrow_and_return_records};

pub fn get_borrow_and_return_record_by_id(id:i32) -> MyQueryResult<BorrowAndReturnRecord> {
    let connection = &mut establish_connection()
        .ok_or(QueryError::DatabaseNoConnect)?;
    borrow_and_return_records::table
        .find(id)
        .first::<BorrowAndReturnRecord>(connection)
        .map_err(|_|QueryError::NoFound)
}

pub fn get_borrow_and_return_record_by_account_id(account_id:i32) -> MyQueryResult<Vec<BorrowAndReturnRecord>> {
    let connection = &mut establish_connection()
        .ok_or(QueryError::DatabaseNoConnect)?;
    borrow_and_return_records::table
        .filter(borrow_and_return_records::account_id.eq(account_id))
        .load::<BorrowAndReturnRecord>(connection)
        .map_err(|_|QueryError::NoFound)
}

pub fn get_borrow_and_return_record_by_book_id(book_id:i32) -> MyQueryResult<Vec<BorrowAndReturnRecord>> {
    let connection = &mut establish_connection()
        .ok_or(QueryError::DatabaseNoConnect)?;
    borrow_and_return_records::table
        .filter(borrow_and_return_records::book_id.eq(book_id))
        .load::<BorrowAndReturnRecord>(connection)
        .map_err(|_| QueryError::NoFound)
}

pub fn get_borrow_and_return_record_by_book_isbn(book_isbn:& str) -> MyQueryResult<Vec<BorrowAndReturnRecord>> {
    let connection = &mut establish_connection()
        .ok_or(QueryError::DatabaseNoConnect)?;
    borrow_and_return_records::table
        .inner_join(books::table)
        .filter(books::isbn.eq(book_isbn))
        .select((
            borrow_and_return_records::id,
            borrow_and_return_records::account_id,
            borrow_and_return_records::book_id,
            borrow_and_return_records::borrow_date,
            borrow_and_return_records::return_date,
        ))
        .load::<BorrowAndReturnRecord>(connection)
        .map_err(|_| QueryError::NoFound)
}

pub fn get_borrow_and_return_record_by_book_title(book_title:& str) -> MyQueryResult<Vec<BorrowAndReturnRecord>> {
    let connection = &mut establish_connection()
        .ok_or(QueryError::DatabaseNoConnect)?;
    borrow_and_return_records::table
        .inner_join(books::table)
        .filter(books::title.eq(book_title))
        .select((
            borrow_and_return_records::id,
            borrow_and_return_records::account_id,
            borrow_and_return_records::book_id,
            borrow_and_return_records::borrow_date,
            borrow_and_return_records::return_date,
        ))
        .load::<BorrowAndReturnRecord>(connection)
        .map_err(|_| QueryError::NoFound)
}
pub fn insert_borrow_and_return_record_by_new_borrow_and_return_record(borrow_and_return_record:&[NewBorrowAndReturnRecord]) ->MyInsertResult<usize> {
    let connection = &mut establish_connection()
        .ok_or(InsertError::DatabaseNoConnect)?;
    insert_into(borrow_and_return_records::table)
        .values(borrow_and_return_record)
        .execute(connection)
        .map_err( |_| InsertError::InsertConflict)
}