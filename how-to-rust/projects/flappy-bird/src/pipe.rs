use macroquad::{color::GREEN, math::Rect, shapes::draw_rectangle, window::screen_height};

pub const PIPE_WIDTH: f32 = 80.0;
pub const GAP_HEIGHT: f32 = 180.0;
pub const PIPE_SPEED: f32 = 3.0;

pub struct Pipe {
    pub x: f32,
    pub gap_y_center: f32,
    pub scored: bool,
}

impl Pipe {
    pub fn new(x: f32, gap_y_center: f32) -> Self {
        Self {
            x,
            gap_y_center,
            scored: false,
        }
    }

    pub fn update(&mut self) {
        self.x -= PIPE_SPEED;
    }

    pub fn draw(&self) {
        draw_rectangle(
            self.x,
            0.0,
            PIPE_WIDTH,
            self.gap_y_center - GAP_HEIGHT / 2.0,
            GREEN,
        );

        draw_rectangle(
            self.x,
            self.gap_y_center + GAP_HEIGHT / 2.0,
            PIPE_WIDTH,
            screen_height() - (self.gap_y_center + GAP_HEIGHT / 2.0),
            GREEN,
        );
    }

    pub fn top_rect(&self) -> Rect {
        Rect::new(
            self.x,
            0.0,
            PIPE_WIDTH,
            self.gap_y_center - GAP_HEIGHT / 2.0,
        )
    }

    pub fn bottom_rect(&self) -> Rect {
        Rect::new(
            self.x,
            self.gap_y_center + GAP_HEIGHT / 2.0,
            PIPE_WIDTH,
            screen_height() - (self.gap_y_center + GAP_HEIGHT / 2.0),
        )
    }

    pub fn is_offscreen(&self) -> bool {
        self.x + PIPE_WIDTH < 0.0
    }
}
