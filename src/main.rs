#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

use std::vec;

use rocket::http::hyper::request::Builder;
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};
use rocket::form::Form;
use rsfbclient::prelude::*;


#[derive(Debug, FromForm, Deserialize, Serialize, Clone)]
struct FormDados {
    dado1: String,
    dado2: String,
    dado3: String,
    dado4: String,
}



#[post("/coleta", data = "<form_dados>")]
fn coleta(form_dados: Form<FormDados>)-> String{    
    let FormDados {dado1, dado2, dado3, dado4} = form_dados.into_inner();
    return format!("dados cadastrados");


}

#[get("/busca_dados")]
fn busca(){
    let conn = rsfbclient::builder_native()
    .with_dyn_link()
    .with_remote()
    .host("http://localhost:8080")
    .db_name("TESTE.fdb")
    .connect();

    let rows: Vec<(String, String)> = conn.query(
        "SELECT dado1, dado2 FROM coleta_dados",(),
    )?;

    


}

#[get("/")]
fn index()->&'static str{
    "Ola, APIMES Veros"
}

#[rocket::main]
async fn main() {
    println!("Hello, world!");
    rocket::build().mount("/", routes![index, coleta, busca]).launch().await.unwrap();
}
