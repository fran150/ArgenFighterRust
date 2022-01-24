use sdl2::{
    Sdl, 
    VideoSubsystem, 
    video::{
        Window, 
        WindowContext
    }, 
    render::{
        Canvas, 
        TextureCreator
    }, 
    pixels::Color
};

use super::GameLoop;

pub struct Game {
    pub context:Sdl,
    pub video_subsystem:VideoSubsystem,
    pub canvas:Canvas<Window>,
    pub texture_creator: TextureCreator<WindowContext>,
    pub game_loop: GameLoop
}

impl Game {
    pub fn new(title:&str, width:u32, height:u32) -> Game {
        let context = sdl2::init()
            .expect("Could not initialize SDL graphics system");

        let video_subsystem = context.video()
            .expect("Could not initialize SDL video subsystem");
    
        let window = video_subsystem.window(title, width, height)
            .opengl()
            .position_centered()
            .build()
            .expect("Could not create application's main window");
    
        let mut canvas = window.into_canvas().build()
            .expect("Could not create main window's canvas"); 
            
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.present();

        let texture_creator = canvas.texture_creator();    

        let game_loop = GameLoop::new();
                
        return Game {
            context,
            video_subsystem,
            canvas,
            texture_creator,
            game_loop
        }
    }

}