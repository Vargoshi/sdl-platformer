use crate::draw::SCREEN_WIDTH;

use super::Floor;

use sdl2::rect::Point;

use sdl2::rect::Rect;

pub struct Body {
    pub rect: Rect,
    pub vel: Point,
    pub on_floor: bool,
    pub on_wall: Option<Dir>,
}

impl Body {
    pub fn step(&mut self, floors: &[Floor], should_collide: bool) {
        self.vel.y += 5;
        self.vel.x += -self.vel.x.signum() * 2;
        self.on_floor = false;
        self.on_wall = None;

        let (offset, did_collide) = if should_collide {
            Self::move_rect(floors, self.rect, Point::new(self.vel.x / 10, 0))
        } else {
            (Point::new(self.vel.x/10,self.vel.y/10), false)
        };
        if did_collide {
            if self.vel.x > 0 {
                self.on_wall = Some(Dir::Right);
            }
            if self.vel.x < 0 {
                self.on_wall = Some(Dir::Left);
            }

            self.vel.x = 0
        }

        self.rect.x += offset.x;

        if self.rect.center().x < 0 {
            self.rect.x += SCREEN_WIDTH as i32;
        }
        if self.rect.center().x > SCREEN_WIDTH as i32 {
            self.rect.x -= SCREEN_WIDTH as i32;
        }

        let (offset, did_collide) = if should_collide {
            Self::move_rect(floors, self.rect, Point::new(0, self.vel.y / 10))
        } else {
            (Point::new(self.vel.x/10,self.vel.y/10), false)
        };
        if did_collide {
            if self.vel.y > 0 {
                self.on_floor = true
            }
            self.vel.y = 0
        }
        self.rect.y += offset.y;
    }

    /// Tries to move rect by the offset but stops on collision.
    /// Returns new offset and whether there was collision.
    fn move_rect(floors: &[Floor], rect: Rect, mut offset: Point) -> (Point, bool) {
        let mut did_collide = false;
        loop {
            let mut moved_rect = rect;
            moved_rect.offset(offset.x, offset.y);
            if Self::is_collision(floors, moved_rect) {
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

    fn is_collision(floors: &[Floor], rect: Rect) -> bool {
        floors.iter().any(|floor| {
            floor.rect.has_intersection(rect)
                || floor.rect.has_intersection(Rect::new(
                    rect.x - SCREEN_WIDTH as i32,
                    rect.y,
                    rect.width(),
                    rect.height(),
                ))
                || floor.rect.has_intersection(Rect::new(
                    rect.x + SCREEN_WIDTH as i32,
                    rect.y,
                    rect.width(),
                    rect.height(),
                ))
        })
    }
}

pub enum Dir {
    Left,
    Right,
}
