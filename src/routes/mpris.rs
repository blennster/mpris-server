use std::time::Duration;

use actix_web::{get, HttpResponse, Responder};
use mpris::PlayerFinder;

use crate::models::player::Player;

#[get("/")]
pub async fn players() -> impl Responder {
    let p = PlayerFinder::new().unwrap().find_active();

    match p {
        Some(p) => HttpResponse::Ok().json(Player::from_mpris(p)),
        None => HttpResponse::NotFound().finish(),
    }
}
