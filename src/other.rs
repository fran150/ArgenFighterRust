
let width = main_window.get_width();
let height = main_window.get_height();

let mut main_window = main_window;

let canvas = &mut main_window.canvas;
let context = &mut main_window.context;

canvas.set_draw_color(Color::RGB(0, 0, 0));
canvas.clear();
canvas.present();

let mut event_pump = context.event_pump()?;
let mut i = 0;
'running: loop {
    i = (i + 1) % 255;
    canvas.set_draw_color(Color::RGB(i, 64, 255 - i));

    let mut rng = rand::thread_rng();

    let x1 = rng.gen_range(0..width) as i32;
    let y1 = rng.gen_range(0..height) as i32;
    let x2 = rng.gen_range(0..width) as i32;
    let y2 = rng.gen_range(0..height) as i32;

    canvas.draw_line(Point::new(x1, y1), Point::new(x2, y2))
        .expect("Couldn't write the line");

    for event in event_pump.poll_iter() {
        match event {
            Event::Quit {..} |
            Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                break 'running;
            },
            _ => {}
        }
    }

    // The rest of the game loop goes here...
    canvas.present();

    ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
}
