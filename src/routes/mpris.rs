use actix_web::{get, post, web, HttpResponse, Responder};
use mpris::PlayerFinder;

use crate::models::player::Player;

fn get_active_player<'a>() -> Option<mpris::Player<'a>> {
    let p = PlayerFinder::new()
        .expect("no d-bus connection")
        .find_active();

    match p {
        Ok(p) => Some(p),
        Err(_) => None,
    }
}

fn get_player_by_id<'a>(identity: &String) -> Option<mpris::Player<'a>> {
    let players = PlayerFinder::new()
        .expect("no d-bus connection")
        .find_all()
        .expect("no players found");

    players
        .into_iter()
        .filter(|p| p.identity().to_lowercase() == identity.to_lowercase())
        .next()
}

#[get("/current")]
pub async fn get_current() -> impl Responder {
    let p = get_active_player();

    match p {
        Some(p) => HttpResponse::Ok().json(Player::from_mpris(&p)),
        None => HttpResponse::NotFound().finish(),
    }
}

#[get("/")]
pub async fn get_players() -> impl Responder {
    let ps = PlayerFinder::new().unwrap().find_all().unwrap();
    let players: Vec<Player> = ps.iter().map(|p| Player::from_mpris(p)).collect();

    HttpResponse::Ok().json(players)
}

#[get("/{id}")]
pub async fn get_player(id: web::Path<String>) -> impl Responder {
    let p = get_player_by_id(&id);

    match p {
        Some(p) => HttpResponse::Ok().json(Player::from_mpris(&p)),
        None => HttpResponse::NotFound().finish(),
    }
}

#[post("/{id}/play")]
pub async fn play(id: web::Path<String>) -> impl Responder {
    let p = get_player_by_id(&id);

    match p {
        Some(p) => {
            p.play().unwrap();
            HttpResponse::Ok().finish()
        }
        None => HttpResponse::NotFound().finish(),
    }
}

#[post("/{id}/pause")]
pub async fn pause(id: web::Path<String>) -> impl Responder {
    let p = get_player_by_id(&id);

    match p {
        Some(p) => {
            p.pause().unwrap();
            HttpResponse::Ok().finish()
        }
        None => HttpResponse::NotFound().finish(),
    }
}

#[post("/{id}/next")]
pub async fn next(id: web::Path<String>) -> impl Responder {
    let p = get_player_by_id(&id);

    match p {
        Some(p) => {
            p.next().unwrap();
            HttpResponse::Ok().finish()
        }
        None => HttpResponse::NotFound().finish(),
    }
}

#[post("/{id}/prev")]
pub async fn prev(id: web::Path<String>) -> impl Responder {
    let p = get_player_by_id(&id);

    match p {
        Some(p) => {
            p.previous().unwrap();
            HttpResponse::Ok().finish()
        }
        None => HttpResponse::NotFound().finish(),
    }
}
