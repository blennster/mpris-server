use std::time::Duration;

use mpris;

const DEFAULT: &str = "Unknown";

#[derive(serde::Serialize)]
pub struct Player {
    identity: String,
    track: String,
    artists: Vec<String>,
    album_art: String,
    volume: f64,
    pos: Duration,
    len: Duration,
    can_play: bool,
    is_playing: bool,
    can_go_next: bool,
    can_go_prev: bool,
    bus_name: String,
}

impl Player {
    pub fn from_mpris(p: &mpris::Player) -> Self {
        let m = p.get_metadata().unwrap();

        Self {
            identity: p.identity().to_string(),
            track: m.title().unwrap_or(DEFAULT).to_string(),
            artists: m
                .album_artists()
                .unwrap_or(&vec![String::from(DEFAULT)])
                .to_vec(),
            album_art: m.art_url().unwrap_or(DEFAULT).to_string(),
            volume: p.get_volume().unwrap_or_default(),
            pos: p.get_position().unwrap_or_default(),
            len: m.length().unwrap_or_default(),
            can_play: p.can_play().unwrap_or(false),
            is_playing: p.get_playback_status().unwrap() == mpris::PlaybackStatus::Playing,
            can_go_next: p.can_go_next().unwrap_or_default(),
            can_go_prev: p.can_go_previous().unwrap_or_default(),
            bus_name: p.bus_name().to_string(),
        }
    }
}
