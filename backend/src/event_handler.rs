use std::sync::RwLock;

use crate::types::{self, Player};
use actix_web::{get, post, web, HttpResponse, Responder};
use types::GameState;

#[post("/")]
pub async fn post_handler(
    game_state: web::Data<RwLock<GameState>>,
    req_body: String,
) -> impl Responder {
    if let Ok(data) = serde_json::from_str::<types::PostBody>(&req_body) {
        let mut gs = game_state.write().unwrap();
        *gs = data;
    } else {
        println!("cannot parse data");
    }
    HttpResponse::Ok()
}

#[get("/")]
pub async fn get(game_state: web::Data<RwLock<GameState>>) -> impl Responder {
    let player = &*game_state.read().unwrap();

    HttpResponse::Ok().body(serde_json::to_string(&player).unwrap())
}

#[get("/player")]
pub async fn get_player(game_state: web::Data<RwLock<GameState>>) -> impl Responder {
    let gs = &*game_state.read().unwrap();
    HttpResponse::Ok().json::<Player>(gs.player.clone())
}
