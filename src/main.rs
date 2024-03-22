use std::path;

#[macro_use] extern crate rocket;

#[path="./Routes/cadastro_user.rs"]
mod cadastro_user;
#[path="./Connection/connection.rs"]
mod connection;

#[rocket::main]
async   fn main() {
    println!("Hello, world!");
    rocket::build().mount("/api", routes![
        cadastro_user::cadastro_user,
        
        
        
        
        ]).launch().await.unwrap();
}