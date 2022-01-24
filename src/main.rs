use engine::{Game, GameLoop, Stepables};
use world::Trump;

mod engine;
mod world;

fn main() -> Result<(), String> {
    let mut game = Game::new("Test", 1024, 768);
    
    let mut gl = std::mem::replace(&mut game.game_loop, GameLoop { objects: Stepables::new() });

    gl.run(&mut game);
    

    Ok(())
}