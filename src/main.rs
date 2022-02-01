mod engine;
mod sys;

use engine::System;

use crate::sys::animator::{ 
    AnimationStep,
    Animator
};

const SPEED: u128 = 85;

fn main() -> Result<(), String> { 
    let mut engine = engine::Main::new();

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

    let animator = Animator::new(steps);
    engine.systems.push(System::RenderSystem(Box::new(animator)));


    engine.run();
    Ok(())
}