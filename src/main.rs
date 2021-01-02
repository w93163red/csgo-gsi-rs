use actix_web::{App, HttpServer};
mod post_handler;
mod types;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(post_handler::post_handler))
        .bind("127.0.0.1:55555")?
        .run()
        .await
}
