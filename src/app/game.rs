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
            Size { w: (sw / 2) - 20, h: 8 },
        );
        game.add_wall(
            Pos {
                x: 0,
                y: sh as i32 - 162,
            },
            Size { w: (sw / 2) - 20, h: 8 },
        );
        game
    }

    fn add_player(&mut self, pos: Pos) {
        self.entities.push(Entity {
            pos: Some(pos),
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
        })
    }

    fn add_enemy(&mut self, pos: Pos) {}

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
        })
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
