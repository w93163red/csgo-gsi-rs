use std::sync::RwLock;

use crate::types::{self};
use actix_web::{
    dev::{HttpResponseBuilder, ResourceDef},
    get, post, web, HttpResponse, Responder,
};
use types::GameState;

#[post("/")]
pub async fn post_handler(
    game_state: web::Data<RwLock<GameState>>,
    raw_data: web::Data<RwLock<String>>,
    req_body: String,
) -> impl Responder {
    match serde_json::from_str::<types::PostBody>(&req_body) {
        Ok(data) => {
            let mut gs = game_state.write().unwrap();
            *gs = data;
        }
        Err(_) => {
            let mut rd = raw_data.write().unwrap();
            *rd = req_body;
        }
    }
    HttpResponse::Ok()
}

#[get("/")]
pub async fn get(game_state: web::Data<RwLock<GameState>>) -> impl Responder {
    let player = &*game_state.read().unwrap();

    HttpResponse::Ok().body(serde_json::to_string(&player).unwrap())
}

#[get("/state")]
pub async fn get_player(game_state: web::Data<RwLock<GameState>>) -> impl Responder {
    let gs = &*game_state.read().unwrap();
    HttpResponse::Ok().json::<GameState>(gs.clone())
}

#[get("/rawdata")]
pub async fn get_raw_data(raw_data: web::Data<RwLock<String>>) -> impl Responder {
    let raw_data = &*raw_data.read().unwrap();
    HttpResponse::Ok().json::<serde_json::Value>(serde_json::from_str(raw_data).unwrap())
}
