mod engine;
mod sys;

fn main() -> Result<(), String> { 
    let mut engine = engine::Main::new();

    engine.run();
    Ok(())
}