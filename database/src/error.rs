use thiserror::Error;

#[derive(Error,Debug)]
pub enum QueryError{
    #[error("Database no connect!")]
    DatabaseNoConnect,
    #[error("Querying it but no found!")]
    NoFound,
}

#[derive(Error,Debug)]
pub enum InsertError{
    #[error("Database no connect!")]
    DatabaseNoConnect,
    #[error("Inserting it but has a conflict!")]
    InsertConflict,
}


pub type MyQueryResult<T>=Result<T,QueryError>;
pub type MyInsertResult<T>=Result<T,InsertError>;