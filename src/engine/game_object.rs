use sdl2::{
    render::Canvas,
    video::Window
};

use super::{input, Textures};

pub trait GameObject {
    fn update(&mut self, t: f64, dt: f64, keyboard: input::Keyboard);
    fn render(&mut self, t: f64, dt: f64, canvas: &mut Canvas<Window>, textures: &mut Textures);
}