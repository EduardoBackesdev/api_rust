#![allow(unused_variables, unused_mut)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate rsfbclient;


use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};
use rocket::{form::Form, http::hyper::header};
use rsfbclient::{prelude::*,FbError};
use serde_json::error;


#[derive(Debug, FromForm, Deserialize, Serialize, Clone)]
struct FormDados {
    dado_1: String,
    dado_2: String,
    dado_3: String,
    dado_4: String,
}

#[post("/coleta", data = "<form_dados>")]
fn coleta(form_dados: Form<FormDados>)-> String{       
        let mut conn = rsfbclient::builder_native()
        .with_dyn_link()
        .with_remote()
        .db_name("C:/Users/User/Desktop/teste/API_RUST.FDB")
        .user("SYSDBA")
        .pass("masterkey")
        .connect().expect("erro");
        let query= conn.execute(&format!("INSERT INTO insert_dados (dado_1,dado_2,dado_3,dado_4) VALUES ({},{},{},{});",form_dados.dado_1, form_dados.dado_2, form_dados.dado_3, form_dados.dado_4),());
        format!("Dados cadastrados")
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