use std::sync::RwLock;

use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use types::GameState;

mod event_handler;
mod types;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let game_state = web::Data::new(RwLock::new(GameState::default()));
    let raw_data = web::Data::new(RwLock::new(String::new()));
    HttpServer::new(move || {
        let cors = Cors::default().allow_any_origin();
        App::new()
            .wrap(cors)
            .app_data(game_state.clone())
            .app_data(raw_data.clone())
            .service(event_handler::post_handler)
            .service(event_handler::get)
            .service(event_handler::get_player)
            .service(event_handler::get_raw_data)
    })
    .bind("0.0.0.0:55555")?
    .run()
    .await
}
