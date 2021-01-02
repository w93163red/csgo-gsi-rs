use crate::types;
use actix_web::{HttpResponse, Responder};

#[post("/")]
pub async fn post_handler(req_body: String) -> impl Responder {
    let data = serde_json::from_str::<types::PostBody>(&req_body).unwrap();
    println!("{:?}", data);
    HttpResponse::Ok()
}
