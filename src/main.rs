mod app;
mod draw;

use app::App;
use draw::{SCREEN_HEIGHT, SCREEN_WIDTH};
use sdl2::{render::Canvas, video::Window, EventPump};

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;

    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("SDL2", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let canvas = window
        .into_canvas()
        .accelerated()
        .build()
        .map_err(|e| e.to_string())?;

    //let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
    // let mut font = ttf_context.load_font(Path::new("assets/OpenSans-Bold.ttf"), 128)?;
    // font.set_style(sdl2::ttf::FontStyle::BOLD);

    let event_pump = sdl_context.event_pump()?;

    game_loop(event_pump, canvas)?;

    Ok(())
}

fn game_loop(mut event_pump: EventPump, canvas: Canvas<Window>) -> Result<(), String> {
    let mut app = App::new(canvas);

    'running: loop {
        for event in event_pump.poll_iter() {
            use sdl2::{event::Event, keyboard::Keycode};
            if let Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } = event
            {
                break 'running;
            }
        }

        app.step(event_pump.keyboard_state());

        app.draw()?;

        ::std::thread::sleep(std::time::Duration::from_millis(1000 / 30));
    }

    Ok(())
}
