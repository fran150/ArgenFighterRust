use crate::gfx::MainWindow;

pub struct Game {
    main_window: MainWindow
}

impl<'a> Game {
    pub fn start() -> Game {
        let main_window = MainWindow::new("test", 1024, 768);

        return Game { 
            main_window
        }
    }
}