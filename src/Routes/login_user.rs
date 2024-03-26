use std::path;


struct dadosEntrada{
    login_user: String,
    password_user: String,
}




#[post ("/login")]
pub fn login_user()-> std::io::Result<()>{

    Ok(())

}