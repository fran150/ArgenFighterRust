use sdl2::{render::Canvas, video::Window};
use super::{Textures, game_object::GameObject};

pub trait GameLevel {
    fn is_level_completed(&self) -> bool;
    fn load(&mut self, canvas: &mut Canvas<Window>, textures: &mut Textures) -> Vec<Box<dyn GameObject>>;
    fn update(&mut self, t: f64, dt: f64, systems: &mut Vec<Box<dyn GameObject>>);
    fn unload(&mut self, systems: &mut Vec<Box<dyn GameObject>>) -> usize;
}