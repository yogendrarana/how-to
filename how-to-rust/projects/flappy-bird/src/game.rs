use macroquad::{
    color::{BLACK, DARKGRAY, RED, SKYBLUE},
    input::{KeyCode, is_key_pressed},
    prelude::rand,
    text::draw_text,
    time::get_frame_time,
    window::{clear_background, screen_width},
};

use crate::bird::Bird;
use crate::pipe::{PIPE_WIDTH, Pipe};

pub enum GameState {
    Menu,
    Playing,
    GameOver,
}

pub struct Game {
    bird: Bird,
    pipes: Vec<Pipe>,
    spawn_timer: f32,
    state: GameState,
    score: u32,
}

impl Game {
    pub fn new() -> Self {
        Self {
            bird: Bird::new(),
            pipes: Vec::new(),
            spawn_timer: 0.0,
            state: GameState::Menu,
            score: 0,
        }
    }

    pub fn update(&mut self) {
        match self.state {
            GameState::Menu => self.update_menu(),
            GameState::Playing => self.update_playing(),
            GameState::GameOver => self.update_game_over(),
        }
    }

    fn update_menu(&mut self) {
        if is_key_pressed(KeyCode::Space) {
            self.state = GameState::Playing;
        }
    }

    fn update_playing(&mut self) {
        self.bird.update();

        self.spawn_timer += get_frame_time();

        if self.spawn_timer > 2.0 {
            self.spawn_timer = 0.0;

            self.pipes
                .push(Pipe::new(screen_width(), rand::gen_range(150.0, 450.0)));
        }

        for pipe in &mut self.pipes {
            pipe.update();
        }

        self.pipes.retain(|pipe| !pipe.is_offscreen());

        for pipe in &mut self.pipes {
            if !pipe.scored && pipe.x + PIPE_WIDTH < self.bird.x {
                pipe.scored = true;
                self.score += 1;
            }

            if self.bird.rect().overlaps(&pipe.top_rect())
                || self.bird.rect().overlaps(&pipe.bottom_rect())
            {
                self.state = GameState::GameOver;
            }
        }

        if self.bird.hit_ground() {
            self.state = GameState::GameOver;
        }
    }

    fn update_game_over(&mut self) {
        if is_key_pressed(KeyCode::R) {
            *self = Self::new();
        }
    }

    pub fn draw(&self) {
        clear_background(SKYBLUE);

        match self.state {
            GameState::Menu => {
                draw_text("FLAPPY BIRD", 170.0, 220.0, 70.0, BLACK);
                draw_text("Press SPACE to Start", 180.0, 300.0, 40.0, DARKGRAY);
            }

            GameState::Playing | GameState::GameOver => {
                for pipe in &self.pipes {
                    pipe.draw();
                }

                self.bird.draw();

                draw_text(&format!("Score: {}", self.score), 20.0, 40.0, 40.0, BLACK);
            }
        }

        if matches!(self.state, GameState::GameOver) {
            draw_text("GAME OVER", 190.0, 220.0, 70.0, RED);
            draw_text(
                &format!("Final Score: {}", self.score),
                240.0,
                290.0,
                40.0,
                BLACK,
            );

            draw_text("Press R to Restart", 210.0, 350.0, 35.0, DARKGRAY);
        }
    }
}
