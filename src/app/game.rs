mod body;

use sdl2::{
    keyboard::{KeyboardState, Scancode},
    pixels::Color,
    rect::{Point, Rect},
    render::Canvas,
    video::Window,
};

use body::Body;

pub struct Game {
    pub player: Player,
    pub floors: Vec<Floor>,
    pub enemies: Vec<Enemy>,
}

impl Game {
    pub fn new() -> Self {
        Self {
            player: Player {
                body: body::Body {
                    rect: Rect::new(200, 200, 32, 64),
                    vel: Point::new(0, 0),
                    on_floor: false,
                },
            },
            floors: vec![
                Floor {
                    rect: Rect::new(100, 500, 500, 32),
                },
                Floor {
                    rect: Rect::new(100, 100, 32, 400),
                },
                Floor {
                    rect: Rect::new(600, 100, 32, 400),
                },
                Floor {
                    rect: Rect::new(100, 400, 300, 32),
                },
            ],
            enemies: vec![
                Enemy {
                    body: Body {
                        rect: Rect::new(300, 250, 32, 64),
                        vel: Point::new(0, 0),
                        on_floor: false,
                    },
                },
                Enemy {
                    body: Body {
                        rect: Rect::new(400, 250, 32, 64),
                        vel: Point::new(0, 0),
                        on_floor: false,
                    },
                },
            ],
        }
    }

    pub fn step(&mut self, ks: KeyboardState) {
        if ks.is_scancode_pressed(Scancode::Space) && self.player.body.on_floor {
            self.player.body.vel.y = -15;
        }
        if ks.is_scancode_pressed(Scancode::Left) {
            self.player.body.vel.x = -10;
        }
        if ks.is_scancode_pressed(Scancode::Right) {
            self.player.body.vel.x = 10;
        }

        self.player.body.step(&self.floors);

        for enemy in &mut self.enemies {
            enemy.body.step(&self.floors);
        }
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        canvas.set_draw_color(Color::RGBA(0, 200, 0, 255));
        canvas.fill_rect(self.player.body.rect)?;
        canvas.set_draw_color(Color::RGBA(200, 200, 200, 255));
        for floor in &self.floors {
            canvas.fill_rect(floor.rect)?;
        }
        canvas.set_draw_color(Color::RGBA(200, 0, 0, 255));
        for enemy in &self.enemies {
            canvas.fill_rect(enemy.body.rect)?;
        }
        Ok(())
    }
}

pub struct Player {
    pub body: body::Body,
}

pub struct Floor {
    pub rect: Rect,
}

pub struct Enemy {
    pub body: body::Body,
}
