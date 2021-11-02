use sdl2::keyboard::{KeyboardState, Scancode};

use crate::app::game::{components::Health, entity::Entity, Game};

pub(crate) fn system(game: &mut Game, ks: KeyboardState) {
    for entity in &mut game.entities {
        if let Entity {
            pos: Some(_),
            size: Some(_),
            vel: Some(vel),
            physics: Some(physics),
            health: Some(health),
            player: Some(_),
            ..
        } = entity
        {
            if health == &Health::Alive {
                if ks.is_scancode_pressed(Scancode::Space) && physics.on_floor {
                    vel.y = -80;
                }
                if ks.is_scancode_pressed(Scancode::Left) && vel.x > -30 {
                    vel.x -= 10;
                }
                if ks.is_scancode_pressed(Scancode::Right) && vel.x < 30 {
                    vel.x += 10;
                }
            }
        }
    }
}
