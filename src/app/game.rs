mod body;

use sdl2::{
    keyboard::{KeyboardState, Scancode},
    pixels::Color,
    rect::{Point, Rect},
    render::Canvas,
    video::Window,
};

use crate::{
    app::game::body::Body,
    draw::{SCREEN_HEIGHT, SCREEN_WIDTH, ZOOM_FACTOR},
};

use self::body::Dir;

pub struct Game {
    player: Player,
    floors: Vec<Floor>,
    enemies: Vec<Enemy>,
}

impl Game {
    pub fn new() -> Self {
        let sw = SCREEN_WIDTH;
        let sh = SCREEN_HEIGHT;
        Self {
            player: Player {
                body: body::Body {
                    rect: Rect::new(sw as i32 / 4, sh as i32 - 40, 16, 21),
                    vel: Point::new(0, 0),
                    on_floor: false,
                    on_wall: None,
                },
                is_dead: false,
            },
            floors: vec![
                Floor {
                    rect: Rect::new(0, sh as i32 - 15, sw, 15),
                },
                Floor {
                    rect: Rect::new((sw as i32 / 2) + 35, sh as i32 - 60 - 3, (sw / 2) - 35, 8),
                },
                Floor {
                    rect: Rect::new(0, sh as i32 - 60 - 3, (sw / 2) - 35, 8),
                },
                Floor {
                    rect: Rect::new(sw as i32 / 4, sh as i32 - 104 - 8, sw / 2, 8),
                },
                Floor {
                    rect: Rect::new(0, sh as i32 - 104, sw / 8, 8),
                },
                Floor {
                    rect: Rect::new(sw as i32 - (sw as i32 / 8), sh as i32 - 104, sw / 8, 8),
                },
                Floor {
                    rect: Rect::new((sw as i32 / 2) + 20, sh as i32 - 162, (sw / 2) - 20, 8),
                },
                Floor {
                    rect: Rect::new(0, sh as i32 - 162, (sw / 2) - 20, 8),
                },
            ],
            enemies: vec![
                Enemy {
                    body: Body {
                        rect: Rect::new(50, 0, 16, 21),
                        vel: Point::new(0, 0),
                        on_floor: false,
                        on_wall: None,
                    },
                    dir: Dir::Right,
                },
                Enemy {
                    body: Body {
                        rect: Rect::new(sw as i32 - 20, 0, 16, 21),
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
                self.player.body.vel.y = -80;
            }
            if ks.is_scancode_pressed(Scancode::Left) && self.player.body.vel.x > -30 {
                self.player.body.vel.x -= 10;
            }
            if ks.is_scancode_pressed(Scancode::Right) && self.player.body.vel.x < 30 {
                self.player.body.vel.x += 10;
            }
        }

        self.player.body.step(&self.floors, !self.player.is_dead);

        for enemy in &mut self.enemies {
            enemy.body.step(&self.floors, true);
            match enemy.dir {
                Dir::Left => {
                    if enemy.body.vel.x > -20 {
                        enemy.body.vel.x -= 3;
                    }
                }
                Dir::Right => {
                    if enemy.body.vel.x < 20 {
                        enemy.body.vel.x += 3;
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
                self.player.body.vel.y = -50;
            }

            if self.player.body.rect.y > SCREEN_HEIGHT as i32 {
                self.player.is_dead = false;
                self.player.body.rect.x = SCREEN_WIDTH as i32 / 2;
                self.player.body.rect.y = 0;
                self.player.body.vel.y = 0;
            }
        }
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        canvas.set_draw_color(Color::RGBA(0, 200, 0, 255));
        canvas.fill_rect(Rect::new(
            self.player.body.rect.x * ZOOM_FACTOR as i32,
            self.player.body.rect.y * ZOOM_FACTOR as i32,
            self.player.body.rect.width() * ZOOM_FACTOR,
            self.player.body.rect.height() * ZOOM_FACTOR,
        ))?;
        //canvas.fill_rect(self.player.body.rect)?;
        let left_rect_offset = self.player.body.rect.x + SCREEN_WIDTH as i32;
        canvas.fill_rect(Rect::new(
            left_rect_offset * ZOOM_FACTOR as i32,
            self.player.body.rect.y * ZOOM_FACTOR as i32,
            self.player.body.rect.width() * ZOOM_FACTOR,
            self.player.body.rect.height() * ZOOM_FACTOR,
        ))?;
        let right_rect_offset = self.player.body.rect.x - SCREEN_WIDTH as i32;
        canvas.fill_rect(Rect::new(
            right_rect_offset * ZOOM_FACTOR as i32,
            self.player.body.rect.y * ZOOM_FACTOR as i32,
            self.player.body.rect.width() * ZOOM_FACTOR,
            self.player.body.rect.height() * ZOOM_FACTOR,
        ))?;

        canvas.set_draw_color(Color::RGBA(200, 200, 200, 255));
        for floor in &self.floors {
            //canvas.fill_rect(floor.rect)?;
            canvas.fill_rect(Rect::new(
                floor.rect.x * ZOOM_FACTOR as i32,
                floor.rect.y * ZOOM_FACTOR as i32,
                floor.rect.width() * ZOOM_FACTOR,
                floor.rect.height() * ZOOM_FACTOR,
            ))?;
        }
        canvas.set_draw_color(Color::RGBA(200, 0, 0, 255));
        for enemy in &self.enemies {
            //canvas.fill_rect(enemy.body.rect)?;
            canvas.fill_rect(Rect::new(
                enemy.body.rect.x * ZOOM_FACTOR as i32,
                enemy.body.rect.y * ZOOM_FACTOR as i32,
                enemy.body.rect.width() * ZOOM_FACTOR,
                enemy.body.rect.height() * ZOOM_FACTOR,
            ))?;
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
