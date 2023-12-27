use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;



pub fn establish_connection() -> Option<MysqlConnection> {
    if dotenv().is_err(){
        println!("dotenv err!");
        return None;
    }
    if let Ok(database_url)=env::var("DATABASE_URL"){
        println!("establish err!");
        MysqlConnection::establish(&database_url).ok()
    }else{
        println!("env err!");
        return None;
    }
}
