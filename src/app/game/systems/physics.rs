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
            physics.on_floor = false;
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
                    let mut on_floor = false;
                    let mut vel = Vel { ..*vel };
                    let mut did_collide = false;

                    if Rect::new(pos.x, pos.y + vel.y / 10, size.w, size.h).has_intersection(
                        Rect::new(other_pos.x, other_pos.y, other_size.w, other_size.h),
                    ) {
                        if vel.y > 0 {
                            on_floor = true;
                        }
                        vel.y = 0;
                        did_collide = true;
                    }

                    if Rect::new(pos.x + vel.x / 10, pos.y + vel.y / 10, size.w, size.h)
                        .has_intersection(Rect::new(
                            other_pos.x,
                            other_pos.y,
                            other_size.w,
                            other_size.h,
                        ))
                    {
                        vel.x = 0;
                        did_collide = true;
                    }

                    if did_collide {
                        collisions.push((index, vel, on_floor));
                    }
                }
            }
        }
    }

    // mutate collided entities
    for (index, vel, on_floor) in collisions {
        let entity = &mut game.entities[index];
        entity.vel = Some(vel);
        entity.physics.as_mut().unwrap().on_floor = on_floor;
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
