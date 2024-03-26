use std::path;

#[macro_use] extern crate rocket;

#[path="./Routes/cadastro_user.rs"]
mod cadastro_user;
#[path="./Connection/connection.rs"]
mod connection;
#[path="./Routes/login_user.rs"]
mod login_user;

#[rocket::main]
async   fn main() {
    println!("Hello, world!");
    rocket::build().mount("/api", routes![
        cadastro_user::cadastro_user,
        login_user::login_user,

        
        
        
        
        ]).launch().await.unwrap();
}