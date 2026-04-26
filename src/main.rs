mod common;
mod input;
mod player;
mod world;
mod menu;
mod game;
mod enemy;

use game::Game;
use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Rust FPS".to_string(),
        window_width: 1280,
        window_height: 720,
        fullscreen: true,
        window_resizable: true,
        platform: miniquad::conf::Platform {
            swap_interval: Some(1),
            ..Default::default()
        },
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game = Game::new();
    loop {
        let delta_time = get_frame_time();
        game.update(delta_time);
        game.render();
        next_frame().await;
    }
}