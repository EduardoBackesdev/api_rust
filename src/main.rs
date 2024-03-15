#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate rsfbclient;

use core::panic;
use std::{fmt::format, vec};

use rocket::{futures::sink::With, http::{hyper::request::Builder, uri::Query}};
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};
use rocket::form::Form;
use rsfbclient::{prelude::*,FbError};
use serde_json::error;


#[derive(Debug, FromForm, Deserialize, Serialize, Clone)]
struct FormDados {
    dado1: String,
    dado2: String,
    dado3: String,
    dado4: String,
}

#[post("/coleta", data = "<form_dados>")]
fn coleta(form_dados: Form<FormDados>)-> String{    
        let mut conn = rsfbclient::builder_native()
        .with_dyn_link()
        .with_remote()
        .host("http://localhost:8000")
        .db_name("TESTE.fdb")
        .user("SYSDBA")
        .pass("masterkey")
        .connect().unwrap();
        let query= conn.execute(&format!("SELECT dado1,dado2,dado3,dado4 FROM COLETA_DADOS VALUES ({},{},{},{});", form_dados.dado1, form_dados.dado2, form_dados.dado3, form_dados.dado4),());
        String::from("dados")
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