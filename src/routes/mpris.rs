use actix_web::{get, post, web, HttpResponse, Responder};
use mpris::PlayerFinder;

use crate::{models::player::Player, AppState};

fn get_all_players<'a>() -> Vec<mpris::Player<'a>> {
    PlayerFinder::new()
        .expect("no d-bus connection")
        .find_all()
        .expect("no players found")
}

pub fn get_active_player() -> Option<Player> {
    let players = get_all_players();
    players
        .iter()
        .map(|p| Player::from_mpris(p))
        .find(|p| p.is_playing)
}

pub fn get_player_by_id<'a>(identity: &String) -> Option<mpris::Player<'a>> {
    let players = get_all_players();

    players
        .into_iter()
        .find(|p| p.identity().to_lowercase() == identity.to_lowercase())
}

#[get("/current")]
pub async fn get_current(data: web::Data<AppState>) -> impl Responder {
    let p = get_active_player();

    match p {
        Some(p) => {
            let mut s = data.current_player_id.lock().unwrap();
            *s = Some(p.identity.clone());
            HttpResponse::Ok().json(p)
        }
        None => {
            let id = data.current_player_id.lock().unwrap().clone();
            if id.is_none() {
                return HttpResponse::NotFound().finish();
            }

            let p = get_player_by_id(&id.unwrap());
            HttpResponse::Ok().json(Player::from_mpris(&p.unwrap()))
        }
    }
}

#[get("/")]
pub async fn get_players() -> impl Responder {
    let players: Vec<Player> = get_all_players()
        .iter()
        .map(|p| Player::from_mpris(p))
        .collect();

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
