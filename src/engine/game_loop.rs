use std::time::Instant;

use super::{Stepables, Game, StepData};

const PPS:u128 = 60;
const FPS:u128 = 60;

pub struct GameLoop {
    pub objects:Stepables
}

impl GameLoop {
    pub fn new() -> GameLoop {
        let objects = Stepables::new();
        return GameLoop {
            objects
        }
    }

    pub fn run(&mut self, game: &mut Game) {
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

        let mut event_pump = game.context.event_pump()
            .expect("Could not get event pump");

        let mut previous_data:Option<StepData> = None;

        'running: loop {
            let current_time = instant.elapsed().as_millis();            
            let step = current_time - previous_time;
            previous_time = current_time;
            
            t += step;
            physics_time += step;
            frame_time += step;
            rate_accumulator += step;

            if physics_time > physics_size {
                let mut data = StepData::new(t, physics_size, &mut event_pump, &mut previous_data);

                self.objects.update(game, &mut data);
                physics_time -= physics_size;

                physics_step_counter += 1;

                if data.should_quit() {
                    break 'running;
                }                

                previous_data = Some(data);
            }

            if frame_time > frame_size {
                self.objects.render(game);
                frame_time -= frame_size;
                
                game.canvas.present();

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