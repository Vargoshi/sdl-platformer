mod components;
mod entity;
mod systems;

use sdl2::{keyboard::KeyboardState, render::Canvas, video::Window};

use crate::{
    app::game::{
        components::{Collision, Health, Physics, Player, Pos, Size, Vel},
        entity::Entity,
    },
    draw::{SCREEN_HEIGHT, SCREEN_WIDTH},
};

pub struct Game {
    entities: Vec<Entity>,
}

impl Game {
    pub fn new() -> Self {
        let sw = SCREEN_WIDTH;
        let sh = SCREEN_HEIGHT;
        Self {
            entities: vec![
                Entity {
                    pos: Some(Pos {
                        x: sw as i32 / 4,
                        y: sh as i32 - 40,
                    }),
                    size: Some(Size { w: 16, h: 21 }),
                    vel: Some(Vel { x: 0, y: 0 }),
                    collision: None,
                    physics: Some(Physics {
                        on_floor: false,
                        on_wall: None,
                        gravity: 5,
                        friction: 2,
                    }),
                    health: Some(Health::Alive),
                    enemy: None,
                    player: Some(Player),
                },
                Entity {
                    pos: Some(Pos {
                        x: 0,
                        y: sh as i32 - 15,
                    }),
                    size: Some(Size { w: sw, h: 15 }),
                    vel: None,
                    collision: Some(Collision),
                    physics: None,
                    health: None,
                    enemy: None,
                    player: None,
                },
                Entity {
                    pos: Some(Pos {
                        x: (sw as i32 / 2) + 35,
                        y: sh as i32 - 60 - 3,
                    }),
                    size: Some(Size {
                        w: (sw / 2) - 35,
                        h: 8,
                    }),
                    vel: None,
                    collision: Some(Collision),
                    physics: None,
                    health: None,
                    enemy: None,
                    player: None,
                },
                Entity {
                    pos: Some(Pos {
                        x: 0,
                        y: sh as i32 - 60 - 3,
                    }),
                    size: Some(Size {
                        w: (sw / 2) - 35,
                        h: 8,
                    }),
                    vel: None,
                    collision: Some(Collision),
                    physics: None,
                    health: None,
                    enemy: None,
                    player: None,
                },
                Entity {
                    pos: Some(Pos {
                        x: sw as i32 / 4,
                        y: sh as i32 - 104 - 8,
                    }),
                    size: Some(Size { w: (sw / 2), h: 8 }),
                    vel: None,
                    collision: Some(Collision),
                    physics: None,
                    health: None,
                    enemy: None,
                    player: None,
                },
            ],
        }
    }

    pub fn step(&mut self, ks: KeyboardState) {
        systems::player_ctrl::system(self, ks);
        systems::physics::system(self);
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        systems::draw_players::system(self, canvas)?;
        systems::draw_enemies::system(self, canvas)?;
        systems::draw_floors::system(self, canvas)?;

        Ok(())
    }
}
