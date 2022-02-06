use sdl2::{
    render::Canvas,
    video::Window
};

use super::{input, Textures};

pub trait Renderable {
    fn render(&mut self, t: u128, dt: u128, canvas: &mut Canvas<Window>, textures: &mut Textures);
}

pub trait Updatable {
    fn update(&mut self, t: u128, dt: u128, keyboard: input::Keyboard);
}

pub enum System {
    RenderSystem(Box<dyn Renderable>),
    UpdateSystem(Box<dyn Updatable>)
}
