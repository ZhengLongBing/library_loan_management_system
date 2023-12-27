use diesel::insert_into;
use diesel::prelude::*;
use crate::error::{InsertError, QueryError, MyQueryResult, MyInsertResult};
use crate::models::{Account, NewAccount};
use crate::connect::establish_connection;
use crate::schema::accounts;


pub fn get_account_by_id(id:i32) -> MyQueryResult<Account> {
    let connection = &mut establish_connection()
        .ok_or(QueryError::DatabaseNoConnect)?;
    accounts::table
        .find(id)
        .first::<Account>(connection)
        .map_err(|_|QueryError::NoFound)
}

pub fn get_account_by_username(username:& str)->MyQueryResult<Account> {
    let connection = &mut establish_connection()
        .ok_or(QueryError::DatabaseNoConnect)?;
    accounts::table
        .filter(accounts::username.eq(username))
        .first::<Account>(connection)
        .map_err(|_|QueryError::NoFound)
}

pub fn get_account_by_email(email:& str)->MyQueryResult<Account>{
    let connection = &mut establish_connection()
        .ok_or(QueryError::DatabaseNoConnect)?;
    accounts::table
        .filter(accounts::email.eq(email))
        .first::<Account>(connection)
        .map_err(|_|QueryError::NoFound)
}
pub fn get_account_by_authority(authority:& str)->MyQueryResult<Vec<Account>>{
    let connection = &mut establish_connection()
        .ok_or(QueryError::DatabaseNoConnect)?;
    accounts::table
        .filter(accounts::authority.eq(authority))
        .load::<Account>(connection)
        .map_err(|_|QueryError::NoFound)
}

pub fn insert_account_by_new_account(account:&[NewAccount]) ->MyInsertResult<usize> {
    let connection = &mut establish_connection()
        .ok_or(InsertError::DatabaseNoConnect)?;
    insert_into(accounts::table)
        .values(account)
        .execute(connection)
        .map_err(|_| InsertError::InsertConflict)
}

pub fn insert_account_by_account(account:&[Account]) ->MyInsertResult<usize> {
    let connection = &mut establish_connection()
        .ok_or(InsertError::DatabaseNoConnect)?;
    insert_into(accounts::table)
        .values(account)
        .execute(connection)
        .map_err(|_| InsertError::InsertConflict)
}