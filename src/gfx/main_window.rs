use sdl2::Sdl;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub struct MainWindow {
    width:u32,
    height:u32,
    pub canvas:Canvas<Window>,
    pub context:Sdl
}

impl MainWindow {
    pub fn new(title:&str, width:u32, height:u32) -> MainWindow {
        let context = sdl2::init()
            .expect("Could not initialize SDL graphics system");

        let video_subsystem = context.video()
            .expect("Could not initialize SDL video subsystem");
    
        let window = video_subsystem.window(title, width, height)
            .position_centered()
            .build()
            .expect("Could not create application's main window");
    
        let canvas = window.into_canvas().build()
            .expect("Could not create main window's canvas");

        return MainWindow {
            width,
            height,
            canvas,
            context
        };
    }

    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }
}