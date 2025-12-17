use actix_web::{App, HttpServer};

use crate::web::routes::{echo, hello};

const PORT: &str = "8080";

pub async fn start_server() -> std::io::Result<()> {
    println!("server started at port: {PORT}");
    HttpServer::new(|| App::new().service(hello).service(echo))
        .bind(format!("127.0.0.1:{}", PORT))?
        .run()
        .await
}
