use sdl2::{video::Window, render::Canvas};

pub struct World(pub i32, pub i32, pub i32);

pub struct Screen(pub i32, pub i32, pub usize);

pub struct Camera(pub i32, pub i32);

impl World {
    pub fn to_screen(&self, camera: Camera, canvas: &Canvas<Window>) -> Screen {
        let cam_x = self.0 - camera.0;
        let cam_z = self.2 - camera.1;

        let (w, h) = canvas.window().drawable_size();

        let scn_origin_x = w as i32 / 2;
        let scn_origin_y = h as i32 / 2;

        Screen(scn_origin_x + cam_z, scn_origin_y + cam_x + self.1, 0)
    }
}