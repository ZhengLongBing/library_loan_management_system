
use diesel::prelude::*;
use chrono::NaiveDateTime;


#[derive(Queryable, Selectable,Identifiable,Debug)]
#[diesel(table_name = super::schema::accounts)]
pub struct Account {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub email: String,
    pub authority:String,
    pub created_at:NaiveDateTime,
}
#[derive(Insertable)]
#[diesel(table_name = super::schema::accounts)]
pub struct NewAccount{
    pub username: String,
    pub password: String,
    pub email: String,
}

#[derive(Queryable, Selectable,Identifiable,Debug)]
#[diesel(table_name = super::schema::books)]
pub struct Book {
    pub id: i32,
    pub title: Option<String>,
    pub author: Option<String>,
    pub isbn: Option<String>,
    pub category:Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = super::schema::books)]
pub struct NewBook {
    pub title:String,
    pub author: Option<String>,
    pub isbn: String,
    pub category: Option<String>,
}


#[derive(Identifiable, Selectable, Queryable, Associations, Debug)]
#[diesel(belongs_to(Book))]
#[diesel(belongs_to(Account))]
#[diesel(table_name = super::schema::borrow_records)]
pub struct BorrowRecord{
    pub id: i32,
    pub book_id: i32,
    pub account_id: i32,
    pub borrow_date: NaiveDateTime,
    pub as_of_date: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = super::schema::borrow_records)]
pub struct NewBorrowRecord{
    pub book_id: i32,
    pub account_id: i32,
    pub as_of_date: NaiveDateTime,
}

#[derive(Identifiable, Selectable, Queryable, Associations, Debug)]
#[diesel(belongs_to(Book))]
#[diesel(belongs_to(Account))]
#[diesel(primary_key(book_id, account_id))]
#[diesel(table_name = super::schema::borrow_and_return_records)]
pub struct BorrowAndReturnRecord {
    pub id: i32,
    pub book_id: i32,
    pub account_id: i32,
    pub borrow_date:NaiveDateTime,
    pub return_date:NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = super::schema::borrow_and_return_records)]
pub struct NewBorrowAndReturnRecord {
    pub book_id: i32,
    pub account_id: i32,
    pub borrow_date:NaiveDateTime,
}
