use crate::app::game::{entity::Entity, Game};

use super::system_utils::move_entity;

/// Change velocity when collisions are detected
pub(crate) fn system(game: &mut Game) {
    let mut collisions = Vec::new();

    for entity in &mut game.entities {
        if let Entity {
            physics: Some(physics),
            ..
        } = entity
        {
            physics.on_floor = false;
            physics.on_left_wall = false;
            physics.on_right_wall = false;
        }
    }

    for (index, entity) in game.entities.iter().enumerate() {
        if let Entity {
            pos: Some(pos),
            size: Some(size),
            vel: Some(vel),
            physics: Some(_),
            ..
        } = entity
        {
            let (offset, col_x, col_y) = move_entity(&game.entities, index, *pos, *size, *vel);

            if col_x || col_y {
                let on_floor = col_y && vel.y > 0.0;
                let on_left_wall = col_x && vel.x < 0.0;
                let on_right_wall = col_x && vel.x > 0.0;

                collisions.push((index, offset, on_floor, on_left_wall, on_right_wall));
            }
        }
    }

    for (index, vel, on_floor, on_left_wall, on_right_wall) in collisions {
        let entity = &mut game.entities[index];
        entity.vel = Some(vel);
        if let Entity {
            physics: Some(physics),
            ..
        } = entity
        {
            physics.on_floor = on_floor;
            physics.on_left_wall = on_left_wall;
            physics.on_right_wall = on_right_wall;
        }
    }
}
