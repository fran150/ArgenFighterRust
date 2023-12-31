mod engine;
mod utils;
mod world;

use engine::{GameLevel, Textures, GameObject};
use sdl2::{render::Canvas, video::Window};

use world::characters::Trump;


struct FirstLevel {
    completed: bool
}

impl FirstLevel {
    fn new() -> Self {
        Self {
            completed: false
        }
    }
}

impl GameLevel for FirstLevel {    
    fn load(&mut self, _canvas: &mut Canvas<Window>, textures: &mut Textures) -> Vec<Box<dyn GameObject>> {
        let mut objects: Vec<Box<dyn GameObject>> = Vec::new();

        let trump = Trump::new(utils::coordinates::World(0,0,0), textures);
        objects.push(Box::new(trump));        
    
        objects
    }

    fn is_level_completed(&self) -> bool {
        self.completed
    }

    fn update(&mut self, _t: f64, _dt: f64, _systems: &mut Vec<Box<dyn GameObject>>) {
    }

    fn unload(&mut self, _systems: &mut Vec<Box<dyn GameObject>>) -> usize {
        1
    }
}

fn main() -> Result<(), String> { 
    let level = FirstLevel::new();
    let mut engine = engine::Game::new(vec![Box::new(level)]);

    engine.run();

    Ok(())
}