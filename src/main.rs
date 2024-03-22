use std::path;

#[macro_use] extern crate rocket;

#[path="./Routes/post.rs"]
mod post_fn;
#[path="./Connection/connection.rs"]
mod connection;

#[rocket::main]
async   fn main() {
    println!("Hello, world!");
    rocket::build().mount("/api", routes![]).launch().await.unwrap();
}