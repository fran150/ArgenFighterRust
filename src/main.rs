mod engine;
mod sys;

use engine::{System, GameLevel, Textures};
use sdl2::{render::Canvas, video::Window};

use crate::sys::animator::{ 
    AnimationStep,
    Animator
};

const SPEED: u128 = 70;

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
    fn load(&mut self, canvas: &mut Canvas<Window>, textures: &mut Textures) -> Vec<System> {
        let mut systems = Vec::new();

        let steps = vec![
            AnimationStep {
                x: 0,
                y: 0,
                w: 100,
                h: 100,
                duration: SPEED
            },
            AnimationStep {
                x: 100,
                y: 0,
                w: 100,
                h: 100,
                duration: SPEED
            },
            AnimationStep {
                x: 200,
                y: 0,
                w: 100,
                h: 100,
                duration: SPEED
            },
            AnimationStep {
                x: 300,
                y: 0,
                w: 100,
                h: 100,
                duration: SPEED
            },
            AnimationStep {
                x: 400,
                y: 0,
                w: 100,
                h: 100,
                duration: SPEED
            },
            AnimationStep {
                x: 500,
                y: 0,
                w: 100,
                h: 100,
                duration: SPEED
            }
        ];

        textures.load_texture("Trump", "assets/trump_run.png");
        
        let animator = Animator::new(steps);
        systems.push(System::RenderSystem(Box::new(animator)));
    
        systems
    }

    fn is_level_completed(&self) -> bool {
        self.completed
    }

    fn update(&mut self, t: u128, dt: u128, systems: &mut Vec<System>) {
        if t > 5000 {
            self.completed = true;
        }
    }

    fn unload(&mut self, systems: &mut Vec<System>) -> usize {
        1
    }
}

fn main() -> Result<(), String> { 
    let level = FirstLevel::new();
    let mut engine = engine::Game::new(vec![Box::new(level)]);
    engine.run();

    Ok(())
}