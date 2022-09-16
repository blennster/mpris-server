use std::time::Duration;

use mpris;

const DEFAULT: &str = "Unknown";

#[derive(serde::Serialize)]
pub struct Player {
    identity: String,
    track: String,
    artist: Vec<String>,
    album_art: String,
    volume: f64,
    pos: Duration,
    len: Duration,
    can_play: bool,
    can_go_next: bool,
    can_go_prev: bool,
}

impl Player {
    pub fn from_mpris(p: mpris::Player) -> Self {
        let m = p.get_metadata().unwrap();

        Self {
            identity: p.identity().to_string(),
            track: m.title().unwrap_or(DEFAULT).to_string(),
            artist: m
                .album_artists()
                .unwrap_or(&vec![String::from(DEFAULT)])
                .to_vec(),
            album_art: m.art_url().unwrap().to_string(),
            volume: p.get_volume().unwrap(),
            pos: p.get_position().unwrap(),
            len: m.length().unwrap(),
            can_play: p.can_play().unwrap(),
            can_go_next: p.can_go_next().unwrap(),
            can_go_prev: p.can_go_previous().unwrap(),
        }
    }
}
