#![allow(unused_variables, unused_mut)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate rsfbclient;

use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};
use rocket::form::Form;
use rsfbclient::{prelude::*,FbError};
use serde_json::error;


#[derive(Debug, FromForm, Deserialize, Serialize, Clone)]
struct FormDados {
    id: String,
    dado1: String,
    dado2: String,
    dado3: String,
    dado4: String,
}

#[post("/coleta", data = "<form_dados>")]
fn coleta(form_dados: Form<FormDados>)-> String{   
        #[cfg(feature = "linking")]{ 
        let mut conn = rsfbclient::builder_native()
        .with_dyn_link()
        .with_remote()
        .host("localhost")
        .db_name("teste.fdb")
        .user("SYSDBA")
        .pass("masterkey")
        .connect().expect("erro");
        let _query= conn.execute(&format!("INSERT INTO coleta_dados (id,dado1,dado2,dado3,dado4) VALUES ({},{},{},{},{});", form_dados.id, form_dados.dado1, form_dados.dado2, form_dados.dado3, form_dados.dado4),());
        }
        format!("{}, {}, {}, {}", form_dados.dado1, form_dados.dado2, form_dados.dado3, form_dados.dado4,  )
        
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