use std::time::Instant;

use crate::gfx::MainWindow;

use super::Stepables;

const PPS:u128 = 60;
const FPS:u128 = 60;

pub struct GameLoop<'a> {
    main_window: MainWindow,
    objects:Stepables<'a>
}

impl<'a> GameLoop<'a> {
    

    pub fn run(&self) {
        let instant = Instant::now();
        let t = 0;

        let mut physics_time = 0;
        let mut frame_time = 0;

        let physics_size = 1000 / PPS;
        let frame_size = 1000 / FPS;

        loop {
            let duration = instant.elapsed();
            let step = duration.as_millis();
            
            let t = t + step;
            physics_time += step;
            frame_time += step;

            if physics_time > physics_size {
                self.objects.update(t, physics_size);
                physics_time -= physics_size;
            }

            if frame_time > frame_size {
                self.objects.render();
                frame_time -= frame_size;
            }
        }

    }
}