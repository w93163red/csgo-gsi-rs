use std::{fs, sync::RwLock};

use crate::types::{self, PostBody};
use actix_web::{get, post, web, HttpResponse, Responder};
use types::GameState;

#[post("/")]
pub async fn post_handler(
    game_state: web::Data<RwLock<GameState>>,
    req_body: String,
) -> impl Responder {
    if let Ok(data) = serde_json::from_str::<types::PostBody>(&req_body) {
        let new_player_info = data.player;
        game_state.write().unwrap().player = new_player_info;
    }
    HttpResponse::Ok()
}

#[get("/")]
pub async fn get(game_state: web::Data<RwLock<GameState>>) -> impl Responder {
    let player = &*game_state.read().unwrap();

    HttpResponse::Ok().body(serde_json::to_string(&player).unwrap())
}
