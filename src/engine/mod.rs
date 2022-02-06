use std::time::Instant;
use sdl2::pixels::Color;

pub mod input;
mod systems;
mod textures;
mod game_level;

pub use game_level::GameLevel;
pub use systems::Renderable;
pub use systems::Updatable;
pub use systems::System;
pub use textures::Textures;

const PPS:u128 = 60;
const FPS:u128 = 60;

pub struct Game {
    active_level: usize,
    pub levels: Vec<Box<dyn GameLevel>>
}

impl Game {
    pub fn new(levels: Vec<Box<dyn GameLevel>>) -> Self {
        Self {
            active_level: 0,
            levels
        }
    }

    pub fn run(&mut self) {
        let context = sdl2::init()
            .expect("Could not initialize SDL2 graphic system");

        let video_subsystem = context
            .video()
            .expect("Could not initialize SDL video subsystem");

        let window = video_subsystem
            .window("ArgenFigher", 1024, 768)
            .opengl()
            .position_centered()
            .build()
            .expect("Could not create application's main window");

        let mut canvas = window
            .into_canvas()
            .build()
            .expect("Could not create main window's canvas");

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.present();

        let texture_creator = canvas.texture_creator();
        
        let mut textures = Textures::new(&texture_creator);

        let mut input_handler = input::InputHandler::new(&context);

        'game_loop: loop {
            let level = &mut self.levels[self.active_level];
            let mut systems = level.load(&mut canvas, &mut textures);

            let instant = Instant::now();
            let mut t = 0;

            let mut previous_time = 0;
            let mut physics_time = 0;
            let mut frame_time = 0;

            let mut rate_accumulator = 0;
            let mut physics_step_counter = 0;
            let mut frame_counter = 0;

            let physics_size = 1000 / PPS;
            let frame_size = 1000 / FPS;

            'level_loop: loop {
                let current_time = instant.elapsed().as_millis();            
                let dt = current_time - previous_time;
                previous_time = current_time;
                
                t += dt;
                physics_time += dt;
                frame_time += dt;
                rate_accumulator += dt;

                if physics_time > physics_size {
                    physics_time -= physics_size;

                    input_handler.handle_events();

                    if input_handler.should_quit() {
                        break 'game_loop;
                    }

                    for system in systems.iter_mut() {
                        if let System::UpdateSystem(updatable) = system {
                            updatable.update(t, physics_size, input_handler.keyboard());
                        }
                    }

                    level.update(t, physics_size, &mut systems);

                    if level.is_level_completed() {
                        let new_level = level.unload(&mut systems);

                        if new_level > 0 && new_level < self.levels.len() {
                            self.active_level = new_level;
                            break 'level_loop;
                        } else {
                            break 'game_loop;
                        }
                    }

                    physics_step_counter += 1;
                }

                if frame_time > frame_size {
                    frame_time -= frame_size;

                    canvas.clear();

                    for system in systems.iter_mut() {
                        if let System::RenderSystem(renderable) = system {
                            renderable.render(t, frame_size, &mut canvas, &mut textures);
                        }
                    }
            
                    canvas.present();
        
                    frame_counter += 1;
                }

                if rate_accumulator > 1000 {
                    println!("{} PPS - {} FPS", physics_step_counter, frame_counter);
                    
                    physics_step_counter = 0;
                    frame_counter = 0;

                    rate_accumulator -= 1000;
                }
            }
        }            
    }
}