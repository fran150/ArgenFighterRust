use sdl2::{
    render::Canvas, 
    video::Window
};

use super::input;

pub trait Renderable {
    fn render(&mut self, t: u128, dt: u128, canvas: &mut Canvas<Window>);
}

pub trait Updatable {
    fn update(&mut self, t: u128, dt: u128, keyboard: input::Keyboard);
}

pub enum System<'a> {
    RenderSystem(&'a mut dyn Renderable),
    UpdateSystem(&'a mut dyn Updatable)
}
