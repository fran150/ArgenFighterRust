use sdl2::{pixels::Color, rect::{Point, Rect}, image::LoadTexture, render::Texture};
use crate::engine::{GameStep, Game, StepData};

pub struct Trump<'a> {
    x:u32,
    y:u32,
    texture:Texture<'a>
}

impl<'a> Trump<'a> {
    pub fn new(game:&'a Game) -> Trump<'a> {
        let texture = game.texture_creator
            .load_texture("assets/trump_run.png")
            .expect("Could not load trump texture");

        let (width, height) = game.canvas.logical_size();
        let x = width / 2;
        let y = height / 2;

        Trump { 
            x, 
            y, 
            texture 
        }
    }
}

impl<'a> GameStep for Trump<'a> {
    fn render(&mut self, game: &mut Game) {
        game.canvas.set_draw_color(Color::RGB(0, 0, 0));
        game.canvas.clear();

        let sprite = Rect::new(0, 0, 100, 100);

        let (width, height) = game.canvas.output_size()
            .expect("could not get canvas size");

        let position = Point::new(0, 0);
        // Treat the center of the screen as the (0, 0) coordinate
        let screen_position = position + Point::new(width as i32 / 2, height as i32 / 2);
        let screen_rect = Rect::from_center(screen_position, sprite.width(), sprite.height());

        game.canvas.copy(&self.texture, sprite, screen_rect)
            .expect("Could not write to screen");

    }

    fn update(&mut self, game:&mut Game, data:&StepData) {
    }
}