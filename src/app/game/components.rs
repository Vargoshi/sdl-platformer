pub(crate) struct Pos {
    pub(crate) x: i32,
    pub(crate) y: i32,
}

pub(crate) struct Size {
    pub(crate) w: u32,
    pub(crate) h: u32,
}

/// velocity in 1/10 pixels
pub(crate) struct Vel {
    pub(crate) x: i32,
    pub(crate) y: i32,
}

pub(crate) struct Collision;

pub(crate) struct Physics {
    pub(crate) on_floor: bool,
    pub(crate) on_wall: Option<Dir>,
    /// acceleration in 1/10 pixels per frame
    pub(crate) gravity: i32,
    /// deceleration in 1/10 pixels per frame
    pub(crate) friction: i32,
}

#[derive(PartialEq)]
pub(crate) enum Health {
    Alive,
    Dead,
}

pub(crate) struct Player;

pub(crate) struct Enemy {
    pub(crate) dir: Dir,
}

pub(crate) enum Dir {
    Left,
    Right,
}
