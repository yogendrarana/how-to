// Explicit imports from Macroquad
use macroquad::{
    color::RED,
    input::{KeyCode, is_key_pressed},
    math::Rect,
    shapes::draw_circle,
    window::screen_height,
};

const GRAVITY: f32 = 0.5;
const FLAP_FORCE: f32 = -8.0;
const RADIUS: f32 = 20.0;

pub struct Bird {
    pub x: f32,
    pub y: f32,
    vertical_velocity: f32,
}

impl Bird {
    pub fn new() -> Self {
        Self {
            x: 150.0,
            y: 300.0,
            vertical_velocity: 0.0,
        }
    }

    pub fn update(&mut self) {
        if is_key_pressed(KeyCode::Space) {
            self.vertical_velocity = FLAP_FORCE;
        }

        self.vertical_velocity += GRAVITY;
        self.y += self.vertical_velocity;

        if self.y < RADIUS {
            self.y = RADIUS;
            self.vertical_velocity = 0.0;
        }
    }

    pub fn draw(&self) {
        draw_circle(self.x, self.y, RADIUS, RED);
    }

    pub fn rect(&self) -> Rect {
        Rect::new(self.x - RADIUS, self.y - RADIUS, RADIUS * 2.0, RADIUS * 2.0)
    }

    pub fn hit_ground(&self) -> bool {
        self.y + RADIUS >= screen_height()
    }
}
