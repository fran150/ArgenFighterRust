use std::collections::HashSet;

use sdl2::keyboard::Keycode;

pub struct Keyboard<'a> {
    #[allow(dead_code)]
    up_keys: &'a HashSet<Keycode>,
    
    #[allow(dead_code)]
    down_keys: &'a HashSet<Keycode>,

    #[allow(dead_code)]
    pressed_keys: &'a HashSet<Keycode>
}

impl<'a> Keyboard<'a> {
    pub(super) fn build(up_keys: &'a HashSet<Keycode>, down_keys: &'a HashSet<Keycode>, pressed_keys: &'a HashSet<Keycode>) -> Keyboard<'a> {
        Keyboard {
            up_keys,
            down_keys,
            pressed_keys
        }
    }

    #[allow(dead_code)]
    pub fn is_up(&self, key: Keycode) -> bool {
        self.up_keys.contains(&key)
    }

    #[allow(dead_code)]
    pub fn is_down(&self, key: Keycode) -> bool {
        self.down_keys.contains(&key)
    }

    #[allow(dead_code)]
    pub fn is_pressed(&self, key: Keycode) -> bool {
        self.pressed_keys.contains(&key)
    }
}