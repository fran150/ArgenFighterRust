use sdl2::{rect::{Rect, Point}, render::Canvas, video::Window};
use crate::engine::Textures;

use super::coordinates;

pub struct AnimationStep {
    pub texture: String,
    pub x: i32,
    pub y: i32,
    pub w: u32,
    pub h: u32,
    pub duration: f64
}

pub struct Animation {
    current_duration: f64,
    current_step: usize,
    steps: Vec<AnimationStep>
}

impl Animation {
    pub fn new(steps: Vec<AnimationStep>) -> Self {
        Self {
            current_duration: steps[0].duration,
            current_step: steps.len(),
            steps
        }
    }

    pub fn from_row(texture: &str, x: i32, y: i32, w: u32, h: u32, count: i32, duration: f64) -> Self {
        let mut steps = Vec::new();
        for index in 0..count {
            let step = AnimationStep {
                texture: String::from(texture),
                x: x + (w as i32 * index),
                y,
                w,
                h,
                duration
            };

            steps.push(step);
        }

        Self::new(steps)
    }

    pub fn draw(&mut self, dt: f64, position: &coordinates::Screen, canvas: &mut Canvas<Window>, textures: &mut Textures) {
        self.current_step %= self.steps.len();

        let step = &self.steps[self.current_step];

        if self.current_duration >= step.duration {
            self.current_duration = 0.0;
            self.current_step += 1;
        }

        let sprite = Rect::new(step.x, step.y, step.w, step.h);

        // Treat the center of the screen as the (0, 0) coordinate
        let screen_position = Point::new(position.0, position.1);
        let screen_rect = Rect::from_center(screen_position, 100, 100);

        let texture = textures.get_texture(step.texture.as_str());

        canvas.copy(texture, sprite, screen_rect)
            .expect("Could not write to screen");
        

        self.current_duration += dt;
    }
}