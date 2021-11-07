mod components;
mod entity;
mod systems;

use std::collections::HashSet;

use parry2d::{
    math::{Real, Vector},
    shape::Cuboid,
};
use sdl2::{keyboard::KeyboardState, pixels::Color, render::Canvas, video::Window};

use crate::{
    app::game::{
        components::{Health, Physics, Player},
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
        let sw = SCREEN_WIDTH as Real;
        let sh = SCREEN_HEIGHT as Real;

        let mut game = Self {
            entities: Vec::new(),
        };

        game.add_player(Vector::new(0.0, 0.0));

        game.add_wall(Vector::new(0.0, sh - 4.0), Vector::new(sw / 2.0, 8.0));

        game.add_enemy(Vector::new(sw / 4.0, 0.0), true);
        game.add_enemy(Vector::new(sw / 2.0, 0.0), false);

        game
    }

    fn add_player(&mut self, pos: Vector<Real>) {
        self.entities.push(Entity {
            pos: Some(pos),
            shape: Some(Cuboid::new(Vector::new(8.0, 10.5))),
            vel: Some(Vector::new(0.0, 0.0)),
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

    fn add_enemy(&mut self, pos: Vector<Real>, jumping: bool) {
        self.entities.push(Entity {
            pos: Some(pos),
            shape: Some(Cuboid::new(Vector::new(8.0, 10.5))),
            vel: Some(Vector::new(0.0, 0.0)),
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

    fn add_wall(&mut self, pos: Vector<Real>, extents: Vector<Real>) {
        self.entities.push(Entity {
            pos: Some(pos),
            shape: Some(Cuboid::new(extents)),
            vel: None,
            physics: Some(Physics {
                on_floor: false,
                on_left_wall: false,
                on_right_wall: false,
                gravity: 0.0,
                friction: 0.0,
                contacts: HashSet::new(),
            }),
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
        systems::enemy_ai::system(self);
        systems::physics::system(self);
        systems::collision::system(self);
        systems::movement::system(self);
        systems::damage::system(self);
        systems::respawn::system(self);
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        systems::rendering::system(self, canvas)?;

        Ok(())
    }
}
