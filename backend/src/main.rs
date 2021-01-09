use std::sync::RwLock;

use actix_web::{web, App, HttpServer};
use types::GameState;

mod event_handler;
mod types;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let game_state = web::Data::new(RwLock::new(GameState::default()));
    HttpServer::new(move || {
        App::new()
            .app_data(game_state.clone())
            .service(event_handler::post_handler)
            .service(event_handler::get)
    })
    .bind("127.0.0.1:55555")?
    .run()
    .await
}
