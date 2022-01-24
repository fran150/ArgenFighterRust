use std::collections::HashSet;

use sdl2::{EventPump, event::Event, keyboard::Keycode};

pub struct StepData {
    pub t: u128,
    pub dt: u128,
    up_keys: HashSet<Keycode>,
    down_keys: HashSet<Keycode>,
    pub(super) pressed_keys: HashSet<Keycode>,
    should_quit: bool
}

impl StepData {
    pub fn new(t: u128, dt: u128, event_pump:&mut EventPump, 
               previous:&mut Option<StepData>) -> StepData {
        let mut down_keys = HashSet::new();
        let mut up_keys = HashSet::new();
        
        let mut pressed_keys = if let Some(previous) = previous {
            previous.pressed_keys.clone()
        } else {
            HashSet::new()
        };

        let should_quit = StepData::handle_events(event_pump, &mut up_keys, &mut down_keys, 
            &mut pressed_keys);

        return StepData {
            t,
            dt,
            up_keys,
            down_keys,
            pressed_keys,
            should_quit
        };
    }

    fn handle_events(event_pump: &mut EventPump, up_keys: &mut HashSet<Keycode>, 
                     down_keys: &mut HashSet<Keycode>,
                     pressed_keys: &mut HashSet<Keycode>) -> bool {

        let mut should_quit = false;
        
        for event in event_pump.poll_event() {
            match event {
                Event::KeyUp { keycode: Some(key), .. } => {
                    up_keys.insert(key);
                    pressed_keys.remove(&key);
                },
                Event::KeyDown { keycode: Some(key), .. } => {
                    down_keys.insert(key);
                    pressed_keys.insert(key);

                    if key == Keycode::Escape {
                        should_quit = true;
                    }
                },
                Event::Quit { .. } => {
                    should_quit = true;
                },
                _ => {}
            }
        }

        return should_quit;
    }

    pub fn key_up(&self, key:Keycode) -> bool {
        self.up_keys.contains(&key)
    }

    pub fn key_down(&self, key:Keycode) -> bool {
        self.down_keys.contains(&key)
    }

    pub fn key_pressed(&self, key:Keycode) -> bool {
        self.pressed_keys.contains(&key)
    }

    pub fn should_quit(&self) -> bool {
        self.should_quit
    }
}