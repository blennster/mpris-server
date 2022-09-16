use std::time::Duration;

use actix_web::{get, HttpResponse, Responder};
use mpris::{Player, PlayerFinder};

use crate::models::player::Player;

fn get_active_player() -> Option<Player> {
    PlayerFinder::new().unwrap().find_active()
}

#[get("/")]
pub async fn get_players() -> impl Responder {
    let p = get_active_player();

    match p {
        Some(p) => HttpResponse::Ok().json(Player::from_mpris(p)),
        None => HttpResponse::NotFound().finish(),
    }
}

#[get("/{id}")]
pub async fn get_player(id: web::Path<String>) -> impl Responder {
    let p = PlayerFinder::new().unwrap().find_by_name(&id);

    match p {
        Some(p) => HttpResponse::Ok().json(Player::from_mpris(p)),
        None => HttpResponse::NotFound().finish(),
    }
}

#[post("/{id}/play")]
pub async fn play(id: web::Path<String>) -> impl Responder {}

#[post("/{id}/pause")]
pub async fn pause(id: web::Path<String>) -> impl Responder {}
