slint::include_modules!();
use database::account::{get_account_by_username, insert_account_by_new_account};
use database::error::{InsertError, QueryError};
use database::models::{NewAccount};

fn main() -> Result<(), slint::PlatformError> {
    let app=App::new()?;
    let app_handle=app.as_weak();
    app.on_log_in(move |username, password| {
         match get_account_by_username(username.as_str()){
             Ok(account) => {
                 if account.password.as_str()==password.as_str(){
                     match account.authority.as_str() {
                         "user" => {
                             std::process::exit(0);
                         }
                         "manager" => {
                             std::process::exit(0);
                         }
                         _ => {
                             panic!("出现未知权限");
                         }
                     }
                 }else{
                     "密码或账号错误！".into()
                 }
             }
             Err(QueryError::NoFound) => {
                 "密码或账号错误！".into()
             }
             Err(QueryError::DatabaseNoConnect)=>{
                 "数据库无响应！".into()
             }
         }
    });
    app.on_register(move |username,email,password, confirm_password| {
         if password==confirm_password{
             let new_account=NewAccount{
                 username: username.to_string(),
                 password: password.to_string(),
                 email: email.to_string(),
             };
             match insert_account_by_new_account(&[new_account]){
                 Ok(_) => {
                     "注册成功！".into()
                 }
                 Err(InsertError::InsertConflict) => {
                     "用户名或邮箱重复！".into()
                 }
                 Err(InsertError::DatabaseNoConnect) => {
                     "数据库无响应！".into()
                 }
             }
         }else{
             "两次密码不一致!".into()
         }
    });
    app.run().unwrap();
    Ok(())
}
