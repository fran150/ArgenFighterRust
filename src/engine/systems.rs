use sdl2::{
    render::Canvas,
    video::Window
};

use super::{input, Textures};

pub struct RenderData<'a> {
    pub t: u128, 
    pub dt: u128, 
    pub canvas: &'a mut Canvas<Window>, 
    pub textures: &'a Textures<'a>
}

pub struct UpdateData<'a> {
    pub t: u128, 
    pub dt: u128, 
    pub keyboard: input::Keyboard<'a>
}

pub trait Renderable {
    fn render(&mut self, data: RenderData);
}

pub trait Updatable {
    fn update(&mut self, data: UpdateData);
}

pub enum System {
    RenderSystem(Box<dyn Renderable>),
    UpdateSystem(Box<dyn Updatable>)
}
