use sdl2::keyboard::Keycode;

use crate::{utils::{coordinates::{self, Camera}, animation::Animation}, engine::{GameObject, Textures}};

const SPEED: f64 = 70.0;
const TEXTURE_NAME: &str = "Trump";

pub struct Trump {
    position: coordinates::World,
    animation_d: Animation,
    animation_u: Animation,
    animation_r: Animation,
    animation_l: Animation,
    direction: i32
}

impl Trump {
    pub fn new(position: coordinates::World, textures: &mut Textures) -> Self {
        textures.load_texture(TEXTURE_NAME, "assets/trump_run.png");
        
        let animation_d = Animation::from_row(TEXTURE_NAME, 0, 0, 100, 100, 6, SPEED);
        let animation_r = Animation::from_row(TEXTURE_NAME, 0, 100, 100, 100, 6, SPEED);
        let animation_u = Animation::from_row(TEXTURE_NAME, 0, 200, 100, 100, 6, SPEED);
        let animation_l = Animation::from_row(TEXTURE_NAME, 0, 300, 100, 100, 6, SPEED);

        Self {
            position,
            animation_d,
            animation_u,
            animation_r,
            animation_l,
            direction: 0
        }
    }
}

impl GameObject for Trump {
    fn update(&mut self, _t: f64, _dt: f64, keyboard: crate::engine::input::Keyboard) {
        if keyboard.is_pressed(Keycode::S) {
            self.direction = 0;
        }

        if keyboard.is_pressed(Keycode::A) {
            self.direction = 1;
        }

        if keyboard.is_pressed(Keycode::W) {
            self.direction = 2;
        }

        if keyboard.is_pressed(Keycode::D) {
            self.direction = 3;
        }
    }

    fn render(&mut self, _t: f64, dt: f64, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, textures: &mut crate::engine::Textures) {
        let camera = Camera(0,0);
        let screen_coordinates = &self.position.to_screen(camera, canvas);

        if self.direction == 0 {
            self.animation_d.draw(dt, screen_coordinates, canvas, textures)
        }

        if self.direction == 1 {
            self.animation_l.draw(dt, screen_coordinates, canvas, textures)
        }

        if self.direction == 2 {
            self.animation_u.draw(dt, screen_coordinates, canvas, textures)
        }

        if self.direction == 3 {
            self.animation_r.draw(dt, screen_coordinates, canvas, textures)
        }
    }
}