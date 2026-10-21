mod bird;
mod game;
mod pipe;

use game::Game;
use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Flappy Bird".to_string(),
        window_width: 800,
        window_height: 600,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game = Game::new();

    loop {
        game.update();
        game.draw();

        next_frame().await;
    }
}
