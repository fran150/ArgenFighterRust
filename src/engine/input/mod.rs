use std::collections::HashSet;
use sdl2::{Sdl, keyboard::Keycode, event::Event, EventPump};

pub use keyboard::Keyboard;

mod keyboard;

pub struct InputHandler {
    up_keys: HashSet<Keycode>,
    down_keys: HashSet<Keycode>,
    pressed_keys: HashSet<Keycode>,
    should_quit: bool,
    event_pump: EventPump
}

impl InputHandler {
    pub fn new(context: &Sdl) -> Self {        
        Self {
            up_keys: HashSet::new(),
            down_keys: HashSet::new(),
            pressed_keys: HashSet::new(),
            should_quit: false,
            event_pump: context.event_pump()
                .expect("Could not get event pump")
        }
    }

    pub(in crate::engine) fn handle_events(&mut self) {
        self.up_keys.clear();
        self.down_keys.clear();

        for event in self.event_pump.poll_event() {
            match event {
                Event::KeyUp { keycode: Some(key), .. } => {
                    self.up_keys.insert(key);
                    self.pressed_keys.remove(&key);
                },
                Event::KeyDown { keycode: Some(key), .. } => {
                    self.down_keys.insert(key);
                    self.pressed_keys.insert(key);

                    if key == Keycode::Escape {
                        self.should_quit = true;
                    }
                },
                Event::Quit { .. } => {
                    self.should_quit = true;
                },
                _ => {}
            }
        }
    }

    pub fn keyboard(&self) -> Keyboard {
        Keyboard::build(&self.up_keys, &self.down_keys, &self.pressed_keys)
    }

    pub fn should_quit(&self) -> bool {
        self.should_quit
    }
}