mod game;



use sdl2::{
    keyboard::{KeyboardState, Scancode},
    pixels::Color,
    render::Canvas,
    video::Window,
};

use self::game::Game;

pub struct App {
    canvas: Canvas<Window>,
    state: State,
}

pub enum State {
    Menu,
    Game(game::Game),
}

impl App {
    pub fn new(canvas: Canvas<Window>) -> Self {
        App {
            canvas,
            state: State::Menu,
        }
    }

    pub fn step(&mut self, ks: KeyboardState) {
        match &mut self.state {
            State::Menu => {
                if ks.is_scancode_pressed(Scancode::Space) {
                    self.state = State::Game(Game::new());
                }
            }
            State::Game(game) => {
                game.step(ks);
            }
        }
    }

    pub fn draw(&mut self) -> Result<(), String> {
        self.canvas.set_draw_color(Color::RGBA(0, 0, 0, 255));
        self.canvas.clear();

        match &self.state {
            State::Menu => {
                // crate::draw::draw_text(self.canvas, ...)
            }
            State::Game(game) => {
                game.draw(&mut self.canvas)?;
            }
        }

        self.canvas.present();
        Ok(())
    }
}
