mod components;
mod entity;
mod systems;

use std::collections::HashSet;

use sdl2::{keyboard::KeyboardState, pixels::Color, render::Canvas, video::Window};

use crate::{
    app::game::{
        components::{Collision, Health, Physics, Player, Pos, Size, Vel},
        entity::Entity,
    },
    draw::{SCREEN_HEIGHT, SCREEN_WIDTH},
};

use self::components::{Dir, Draw, Enemy};

pub struct Game {
    entities: Vec<Entity>,
}

impl Game {
    pub fn new() -> Self {
        let sw = SCREEN_WIDTH;
        let sh = SCREEN_HEIGHT;
        let mut game = Self {
            entities: Vec::new(),
        };
        game.add_player(Pos {
            x: sw as i32 / 4,
            y: sh as i32 - 40,
        });
        game.add_wall(
            Pos {
                x: 0,
                y: sh as i32 - 15,
            },
            Size { w: sw, h: 15 },
        );
        game.add_wall(
            Pos {
                x: 0,
                y: sh as i32 - 60 - 3,
            },
            Size { w: 8, h: 40 },
        );
        game.add_wall(
            Pos {
                x: (sw as i32 / 2) + 35,
                y: sh as i32 - 60 - 3,
            },
            Size {
                w: (sw / 2) - 35,
                h: 8,
            },
        );
        game.add_wall(
            Pos {
                x: 0,
                y: sh as i32 - 60 - 3,
            },
            Size {
                w: (sw / 2) - 35,
                h: 8,
            },
        );
        game.add_wall(
            Pos {
                x: sw as i32 / 4,
                y: sh as i32 - 104 - 8,
            },
            Size { w: (sw / 2), h: 8 },
        );
        game.add_wall(
            Pos {
                x: 0,
                y: sh as i32 - 104,
            },
            Size { w: (sw / 8), h: 8 },
        );
        game.add_wall(
            Pos {
                x: sw as i32 - (sw as i32 / 8),
                y: sh as i32 - 104,
            },
            Size { w: (sw / 8), h: 8 },
        );
        game.add_wall(
            Pos {
                x: (sw as i32 / 2) + 20,
                y: sh as i32 - 162,
            },
            Size {
                w: (sw / 2) - 20,
                h: 8,
            },
        );
        game.add_wall(
            Pos {
                x: 0,
                y: sh as i32 - 162,
            },
            Size {
                w: (sw / 2) - 20,
                h: 8,
            },
        );
        game.add_enemy(
            Pos {
                x: sw as i32 / 4,
                y: 0,
            },
            true,
        );
        game.add_enemy(
            Pos {
                x: sw as i32 / 2,
                y: 0,
            },
            false,
        );
        game
    }

    fn add_player(&mut self, pos: Pos) {
        self.entities.push(Entity {
            pos: Some(pos),
            size: Some(Size { w: 16, h: 21 }),
            vel: Some(Vel { x: 0.0, y: 0.0 }),
            collision: None,
            physics: Some(Physics {
                on_floor: false,
                on_left_wall: false,
                on_right_wall: false,
                gravity: 0.5,
                friction: 0.2,
                contacts: HashSet::new(),
            }),
            health: Some(Health::Alive),
            enemy: None,
            player: Some(Player),
            draw: Some(Draw {
                color: Color::RGBA(0, 200, 0, 255),
            }),
        })
    }

    fn add_enemy(&mut self, pos: Pos, jumping: bool) {
        self.entities.push(Entity {
            pos: Some(pos),
            size: Some(Size { w: 16, h: 21 }),
            vel: Some(Vel { x: 0.0, y: 0.0 }),
            collision: Some(Collision),
            physics: Some(Physics {
                on_floor: false,
                on_left_wall: false,
                on_right_wall: false,
                gravity: 0.5,
                friction: 0.2,
                contacts: HashSet::new(),
            }),
            health: Some(Health::Alive),
            enemy: Some(Enemy {
                dir: Dir::Left,
                jumping,
            }),
            player: None,
            draw: Some(Draw {
                color: Color::RGBA(200, 0, 0, 255),
            }),
        })
    }

    fn add_wall(&mut self, pos: Pos, size: Size) {
        self.entities.push(Entity {
            pos: Some(pos),
            size: Some(size),
            vel: None,
            collision: Some(Collision),
            physics: None,
            health: None,
            enemy: None,
            player: None,
            draw: Some(Draw {
                color: Color::RGBA(200, 200, 200, 255),
            }),
        })
    }

    pub fn step(&mut self, ks: KeyboardState) {
        systems::player_ctrl::system(self, ks);
        systems::physics::system(self);
        systems::collisions::system(self);
        systems::movement::system(self);
        systems::damage::system(self);
        systems::respawn::system(self);
        systems::stuck_detection::system(self);
        systems::enemy_ai::system(self);
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        systems::rendering::system(self, canvas)?;

        Ok(())
    }
}
