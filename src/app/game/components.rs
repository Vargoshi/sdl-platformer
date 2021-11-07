use std::collections::HashSet;

use sdl2::pixels::Color;

#[derive(Debug, Clone)]
pub(crate) struct Physics {
    pub(crate) on_floor: bool,
    pub(crate) on_left_wall: bool,
    pub(crate) on_right_wall: bool,
    pub(crate) gravity: f32,
    pub(crate) friction: f32,
    pub(crate) contacts: HashSet<usize>,
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
    pub(crate) jumping: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum Dir {
    Left,
    Right,
}

pub(crate) struct Draw {
    pub color: Color,
}
