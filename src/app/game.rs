use sdl2::{
    keyboard::{KeyboardState, Scancode},
    rect::{Point, Rect},
};

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
        self.player.velocity.x += -self.player.velocity.x.signum();

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

        let (offset, did_collide) =
            self.move_rect(self.player.rect, Point::new(self.player.velocity.x, 0));
        if did_collide {
            self.player.velocity.x = 0
        }
        self.player.rect.x += offset.x;

        let (offset, did_collide) =
            self.move_rect(self.player.rect, Point::new(0, self.player.velocity.y));
        if did_collide {
            if self.player.velocity.y > 0 {
                self.player.on_floor = true
            }
            self.player.velocity.y = 0
        }
        self.player.rect.y += offset.y;
    }

    /// Tries to move rect by the offset but stops on collision.
    /// Returns new offset and whether there was collision.
    fn move_rect(&self, rect: Rect, mut offset: Point) -> (Point, bool) {
        let mut did_collide = false;
        loop {
            let mut moved_rect = rect;
            moved_rect.offset(offset.x, offset.y);
            if self.is_collision(moved_rect) {
                did_collide = true;
            } else {
                break;
            }
            offset.x -= offset.x.signum();
            offset.y -= offset.y.signum();
            if offset.x == 0 && offset.y == 0 {
                break;
            }
        }
        (offset, did_collide)
    }

    fn is_collision(&self, rect: Rect) -> bool {
        self.floors
            .iter()
            .any(|floor| floor.rect.has_intersection(rect))
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
