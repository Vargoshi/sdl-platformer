mod body;

use sdl2::{
    keyboard::{KeyboardState, Scancode},
    pixels::Color,
    rect::{Point, Rect},
    render::Canvas,
    video::Window,
};

use self::body::{Body, Dir};

pub struct Game {
    player: Player,
    floors: Vec<Floor>,
    enemies: Vec<Enemy>,
}

impl Game {
    pub fn new() -> Self {
        Self {
            player: Player {
                body: body::Body {
                    rect: Rect::new(200, 200, 32, 64),
                    vel: Point::new(0, 0),
                    on_floor: false,
                    on_wall: None,
                },
                is_dead: false,
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
                        on_wall: None,
                    },
                    dir: Dir::Left,
                },
                Enemy {
                    body: Body {
                        rect: Rect::new(400, 250, 32, 64),
                        vel: Point::new(0, 0),
                        on_floor: false,
                        on_wall: None,
                    },
                    dir: Dir::Left,
                },
            ],
        }
    }

    pub fn step(&mut self, ks: KeyboardState) {
        if !self.player.is_dead {
            if ks.is_scancode_pressed(Scancode::Space) && self.player.body.on_floor {
                self.player.body.vel.y = -15;
            }
            if ks.is_scancode_pressed(Scancode::Left) && self.player.body.vel.x > -10 {
                self.player.body.vel.x -= 2;
            }
            if ks.is_scancode_pressed(Scancode::Right) && self.player.body.vel.x < 10 {
                self.player.body.vel.x += 2;
            }
        }

        self.player.body.step(&self.floors, !self.player.is_dead);

        for enemy in &mut self.enemies {
            enemy.body.step(&self.floors, true);
            match enemy.dir {
                Dir::Left => {
                    if enemy.body.vel.x > -3 {
                        enemy.body.vel.x -= 2;
                    }
                }
                Dir::Right => {
                    if enemy.body.vel.x < 3 {
                        enemy.body.vel.x += 2;
                    }
                }
            }
            match enemy.body.on_wall {
                None => {}
                Some(Dir::Left) => {
                    enemy.dir = Dir::Right;
                }
                Some(Dir::Right) => {
                    enemy.dir = Dir::Left;
                }
            }
            if enemy.body.rect.has_intersection(self.player.body.rect) && !self.player.is_dead {
                self.player.is_dead = true;
                self.player.body.vel.x = 0;
                self.player.body.vel.y = -5;
            }

            if self.player.body.rect.y > 600 {
                self.player.is_dead = false;
                self.player.body.rect.y = 100;
            }
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

struct Player {
    body: body::Body,
    is_dead: bool,
}

pub struct Floor {
    rect: Rect,
}

struct Enemy {
    body: body::Body,
    dir: Dir,
}
