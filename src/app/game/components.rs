use std::ops::{Add, AddAssign, Mul, MulAssign};

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
            x: self.x + rhs.x as i32,
            y: self.y + rhs.y as i32,
        }
    }
}

impl AddAssign<Vel> for Pos {
    fn add_assign(&mut self, rhs: Vel) {
        self.x += rhs.x as i32;
        self.y += rhs.y as i32;
    }
}

impl Mul<f32> for Vel {
    type Output = Vel;

    fn mul(self, rhs: f32) -> Self::Output {
        Vel {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl MulAssign<f32> for Vel {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct Size {
    pub(crate) w: u32,
    pub(crate) h: u32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct Vel {
    pub(crate) x: f32,
    pub(crate) y: f32,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Collision;

#[derive(Debug, Clone, Copy)]
pub(crate) struct Physics {
    pub(crate) on_floor: bool,
    pub(crate) on_left_wall: bool,
    pub(crate) on_right_wall: bool,
    pub(crate) gravity: f32,
    pub(crate) friction: f32,
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
