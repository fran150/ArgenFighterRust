use sdl2::rect::{Rect, Point};
use crate::engine::{Renderable, RenderData};

pub struct AnimationStep {
    pub x: i32,
    pub y: i32,
    pub w: u32,
    pub h: u32,
    pub duration: u128
}

pub struct Animator {
    current_duration: u128,
    current_step: usize,
    steps: Vec<AnimationStep>
}

impl Animator {
    pub fn new(steps: Vec<AnimationStep>) -> Self {
        Self {
            current_duration: steps[0].duration,
            current_step: steps.len(),
            steps
        }
    }
}

impl Renderable for Animator {
    fn render(&mut self, data: RenderData) {
        self.current_step %= self.steps.len();

        let step = &self.steps[self.current_step];

        if self.current_duration >= step.duration {
            self.current_duration = 0;
            self.current_step += 1;
        }

        let (screen_width, screen_height) = data.canvas.output_size()
            .expect("could not get canvas size");

        let sprite = Rect::new(step.x, step.y, step.w, step.h);

        let position = Point::new(0, 0);
        // Treat the center of the screen as the (0, 0) coordinate
        let screen_position = position + Point::new(screen_width as i32 / 2, screen_height as i32 / 2);
        let screen_rect = Rect::from_center(screen_position, 100, 100);

        let texture = data.textures.get_texture("Trump");

        data.canvas.copy(texture, sprite, screen_rect)
            .expect("Could not write to screen");
        

        self.current_duration += data.dt;
    }
}
