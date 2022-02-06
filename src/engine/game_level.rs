use sdl2::{render::Canvas, video::Window};
use super::{Textures, System};

pub trait GameLevel {
    fn is_level_completed(&self) -> bool;
    fn load(&mut self, canvas: &mut Canvas<Window>, textures: &mut Textures) -> Vec<System>;
    fn update(&mut self, t: u128, dt: u128, systems: &mut Vec<System>);
    fn unload(&mut self, systems: &mut Vec<System>) -> usize;
}