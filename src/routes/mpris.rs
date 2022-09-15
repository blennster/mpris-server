use std::time::Duration;

use actix_web::{get, HttpResponse, Responder};
use mpris::PlayerFinder;

#[get("/")]
pub async fn players() -> impl Responder {
    let p = PlayerFinder::new().unwrap().find_active().unwrap();

    #[derive(serde::Serialize)]
    struct Player {
        identity: String,
        track: String,
        artist: Vec<String>,
        volume: f64,
        pos: Duration,
        len: Duration,
        can_play: bool,
        can_go_next: bool,
        can_go_prev: bool,
    }

    let m = p.get_metadata().unwrap();

    let player = Player {
        identity: p.identity().to_string(),
        track: m.title().unwrap_or("Unknown").to_string(),
        artist: m
            .album_artists()
            .unwrap_or(&vec![String::from("Unknown")])
            .to_vec(),
        volume: p.get_volume().unwrap(),
        pos: p.get_position().unwrap(),
        len: m.length().unwrap(),
        can_play: p.can_play().unwrap(),
        can_go_next: p.can_go_next().unwrap(),
        can_go_prev: p.can_go_previous().unwrap(),
    };

    HttpResponse::Ok().json(player)
}
