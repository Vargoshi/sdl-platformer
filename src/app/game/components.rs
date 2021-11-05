use std::ops::{Add, Mul};

use sdl2::pixels::Color;

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct Pos {
    pub(crate) x: i32,
    pub(crate) y: i32,
}

impl Add<Vel> for Pos {
    type Output = Pos;

    fn add(self, rhs: Vel) -> Self::Output {
        Pos {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Mul<i32> for Vel {
    type Output = Vel;

    fn mul(self, rhs: i32) -> Self::Output {
        Vel {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct Size {
    pub(crate) w: u32,
    pub(crate) h: u32,
}

/// velocity in 1/10 pixels
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct Vel {
    pub(crate) x: i32,
    pub(crate) y: i32,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Collision;

#[derive(Debug, Clone, Copy)]
pub(crate) struct Physics {
    pub(crate) on_floor: bool,
    pub(crate) on_left_wall: bool,
    pub(crate) on_right_wall: bool,
    /// acceleration in 1/10 pixels per frame
    pub(crate) gravity: i32,
    /// deceleration in 1/10 pixels per frame
    pub(crate) friction: i32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum Health {
    Alive,
    Dead,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Player;

#[derive(Debug, Clone, Copy)]
pub(crate) struct Enemy {
    pub(crate) dir: Dir,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum Dir {
    Left,
    Right,
}

pub(crate) struct Draw {
    pub color: Color,
}
