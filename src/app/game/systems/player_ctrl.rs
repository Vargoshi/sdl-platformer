use sdl2::keyboard::{KeyboardState, Scancode};

use crate::app::game::{components::Health, entity::Entity, Game};

pub(crate) fn system(game: &mut Game, ks: KeyboardState) {
    for entity in &mut game.entities {
        if let Entity {
            pos: Some(_),
            shape: Some(_),
            vel: Some(vel),
            physics: Some(physics),
            health: Some(health),
            player: Some(_),
            ..
        } = entity
        {
            if health == &Health::Alive {
                if ks.is_scancode_pressed(Scancode::Space) && physics.on_floor {
                    vel.y = -8.0;
                }
                if ks.is_scancode_pressed(Scancode::Left) && vel.x > -3.0 {
                    vel.x -= 1.0;
                }
                if ks.is_scancode_pressed(Scancode::Right) && vel.x < 3.0 {
                    vel.x += 1.0;
                }
            }

            // println!(
            //     "{:?} {} {} {}",
            //     physics.contacts,
            //     if physics.on_floor { "floor" } else { "" },
            //     if physics.on_left_wall {
            //         "left_wall"
            //     } else {
            //         ""
            //     },
            //     if physics.on_right_wall {
            //         "right_wall"
            //     } else {
            //         ""
            //     },
            // );
        }
    }
}
