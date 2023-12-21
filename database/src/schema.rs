// @generated automatically by Diesel CLI.

diesel::table! {
    accounts (id) {
        id -> Integer,
        #[max_length = 50]
        username -> Varchar,
        #[max_length = 100]
        password -> Varchar,
        #[max_length = 100]
        email -> Varchar,
        #[max_length = 20]
        authority -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::table! {
    books (id) {
        id -> Integer,
        #[max_length = 255]
        title -> Varchar,
        #[max_length = 100]
        author -> Nullable<Varchar>,
        #[max_length = 20]
        isbn -> Varchar,
        #[max_length = 50]
        category -> Nullable<Varchar>,
    }
}

diesel::table! {
    borrow_and_return_records (id) {
        id -> Integer,
        account_id -> Integer,
        book_id -> Integer,
        borrow_date -> Timestamp,
        return_date -> Timestamp,
    }
}

diesel::table! {
    borrow_records (id) {
        id -> Integer,
        account_id -> Integer,
        book_id -> Integer,
        borrow_date -> Timestamp,
        as_of_date -> Timestamp,
    }
}

diesel::joinable!(borrow_and_return_records -> accounts (account_id));
diesel::joinable!(borrow_and_return_records -> books (book_id));
diesel::joinable!(borrow_records -> accounts (account_id));
diesel::joinable!(borrow_records -> books (book_id));

diesel::allow_tables_to_appear_in_same_query!(
    accounts,
    books,
    borrow_and_return_records,
    borrow_records,
);
