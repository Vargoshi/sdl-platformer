use sdl2::pixels::Color;

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct Pos {
    pub(crate) x: i32,
    pub(crate) y: i32,
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
    pub(crate) on_wall: Option<Dir>,
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

#[derive(Debug, Clone, Copy)]
pub(crate) enum Dir {
    Left,
    Right,
}

pub(crate) struct Draw {
    pub color: Color,
}