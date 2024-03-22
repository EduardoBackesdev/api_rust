// Cadastro usuarios-> Codigo para cadastrar usuarios no banco de dados
// By:EduardoBackesDev

use std::path;

use rocket::form::Form;
use rsfbclient::Execute;

#[path = "../Connection/connection.rs"]
mod connection;

#[derive(FromForm)]
pub struct user{
    usuario_login: String,
    usuario_password: String,
}

#[post("/cadastro", data = "<form>")]
pub fn post_fn (form:Form<user>)-> std::io::Result<()>{
    let db_path="C:/Users/User/Desktop/teste/API_EDUARDO.FDB";
    let mut conn = connection::connection(&db_path)
    .ok()
    .unwrap();
    conn.execute(
        &format!("INSERT INTO usuarios (USUARIO_LOGIN, USUARIO_PASSWORD) VALUES ('{}', '{}')",
        &form.usuario_login, &form.usuario_password ),
        ()
    )
    .ok()
    .expect("Erro ao cadastrar usuario");
    Ok(())
}