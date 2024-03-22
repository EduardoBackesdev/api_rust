
use std::path;

use crate::connection::connection;
use rocket::form::Form;
use rocket::State;
use rsfbclient::Execute;

#[path = "../Connection/connection.rs"]
mod connection;

#[derive(FromForm)]
pub struct user{
    login: String,
    password: String,
}

#[post("/cadastro", data = "<form>")]
pub fn post_fn (form:Form<user>)-> std::io::Result<()>{
    let db_path:String= String::from("C:/Users/User/Desktop/teste/API_EDUARDO.FDB");
    let mut conn = connection::connection(db_path)
    .ok()
    .unwrap();

    conn.execute(&format!("INSERT INTO usuarios (login, password) VALUES ('{}', '{}')",&form.login, &form.password ),())
    .ok()
    .expect("Erro ao cadastrar usuario");
    Ok(())

}