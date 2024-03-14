#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};
use rocket::form::Form;


#[derive(Debug, FromForm, Deserialize, Serialize)]
struct FormDados {
    dado1: String,
    dado2: String,
    dado3: String,
    dado4: String,
}

#[post("/coleta", data = "<form_dados>")]
fn coleta (form_dados: Form<FormDados>)-> String{
    let FormDados {dado1, dado2, dado3, dado4} = form_dados.into_inner();
    format!("dado1: {}, dado2: {}, dado3: {}, dado4: {}",dado1, dado2, dado3, dado4)
}

#[get("/")]
fn index()->&'static str{
    "Ola, APIMES Veros"
}

#[rocket::main]
async fn main() {
    println!("Hello, world!");
    rocket::build().mount("/", routes![index, coleta]).launch().await.unwrap();
}
