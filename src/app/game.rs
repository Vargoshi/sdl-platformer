use sdl2::{
    keyboard::{KeyboardState, Scancode},
    rect::{Point, Rect},
};
use std::cmp::Ordering;

pub struct Game {
    pub player: Player,
    pub floors: Vec<Floor>,
}

impl Game {
    pub fn new() -> Self {
        Self {
            player: Player {
                rect: Rect::new(200, 200, 32, 64),
                velocity: Point::new(0, 0),
                on_floor: false,
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
        }
    }

    pub fn step(&mut self, ks: KeyboardState) {
        self.player.velocity.y += 1;
        match self.player.velocity.x.cmp(&0) {
            Ordering::Less => {
                self.player.velocity.x += 1;
            }
            Ordering::Greater => {
                self.player.velocity.x -= 1;
            }
            Ordering::Equal => {}
        }

        if ks.is_scancode_pressed(Scancode::Space) && self.player.on_floor {
            self.player.velocity.y = -15;
        }
        if ks.is_scancode_pressed(Scancode::Left) {
            self.player.velocity.x = -10;
        }
        if ks.is_scancode_pressed(Scancode::Right) {
            self.player.velocity.x = 10;
        }

        self.player.on_floor = false;

        if self.is_collision(Point::new(self.player.velocity.x, 0)) {
            self.player.velocity.x = 0;
        }
        self.player.rect.x += self.player.velocity.x;
        if self.is_collision(Point::new(0, self.player.velocity.y)) {
            if self.player.velocity.y > 0 {
                self.player.on_floor = true;
            }
            self.player.velocity.y = 0;
        }
        self.player.rect.y += self.player.velocity.y;
    }

    pub fn is_collision(&self, offset: Point) -> bool {
        let mut moved_rect = self.player.rect;
        moved_rect.offset(offset.x, offset.y);
        for floor in &self.floors {
            if floor.rect.has_intersection(moved_rect) {
                return true;
            }
        }
        false
    }
}

pub struct Player {
    pub rect: Rect,
    pub velocity: Point,
    pub on_floor: bool,
}

pub struct Floor {
    pub rect: Rect,
}
