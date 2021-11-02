use sdl2::rect::Rect;

use crate::app::game::{components::Vel, entity::Entity, Game};

pub(crate) fn system(game: &mut Game) {
    let mut collisions = Vec::new();

    // apply gravity and friction to velocity
    for entity in &mut game.entities {
        if let Entity {
            pos: Some(_),
            size: Some(_),
            vel: Some(vel),
            physics: Some(physics),
            collision: None,
            ..
        } = entity
        {
            vel.y += physics.gravity;
            vel.x += -vel.x.signum() * physics.friction;
        }
    }

    // detect collisions and insert to temporary vector
    for (index, entity) in game.entities.iter().enumerate() {
        if let Entity {
            pos: Some(pos),
            size: Some(size),
            vel: Some(vel),
            physics: Some(_),
            collision: None,
            ..
        } = entity
        {
            for other_entity in &game.entities {
                if let Entity {
                    pos: Some(other_pos),
                    size: Some(other_size),
                    collision: Some(_),
                    physics: None,
                    ..
                } = other_entity
                {
                    if Rect::new(pos.x, pos.y + vel.y / 10, size.w, size.h).has_intersection(
                        Rect::new(other_pos.x, other_pos.y, other_size.w, other_size.h),
                    ) {
                        collisions.push((index, Vel { x: vel.x, y: 0 }));
                        break;
                    }

                    if Rect::new(pos.x + vel.x / 10, pos.y, size.w, size.h).has_intersection(
                        Rect::new(other_pos.x, other_pos.y, other_size.w, other_size.h),
                    ) {
                        collisions.push((index, Vel { x: 0, y: vel.y }));
                        break;
                    }
                }
            }
        }
    }

    // mutate collided entities
    for (index, vel) in collisions {
        game.entities[index].vel = Some(vel);
    }

    // add velocity to position
    for entity in &mut game.entities {
        if let Entity {
            pos: Some(pos),
            size: Some(_),
            vel: Some(vel),
            physics: Some(_),
            collision: None,
            ..
        } = entity
        {
            pos.x += vel.x / 10;
            pos.y += vel.y / 10;
        }
    }
}
