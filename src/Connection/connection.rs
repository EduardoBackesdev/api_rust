use rsfbclient_native::NativeFbClient;
use rsfbclient::{charset::WIN_1252, FbError, DynLink, Connection};


pub fn connection (db_path: String)->Result<Connection<NativeFbClient<DynLink>>, FbError> {
    let conn: Connection<NativeFbClient<DynLink>> = rsfbclient::builder_native()
    .with_dyn_link()
    .with_remote()
    .pass("masterkey")
    .db_name(db_path)
    .host("localhost")
    .port(3050)
    .user("sysdba")
    .charset(WIN_1252)
    .connect()
    .ok()
    .unwrap();
    Ok(conn)
}