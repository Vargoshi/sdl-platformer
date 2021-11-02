use sdl2::rect::Rect;

use crate::app::game::{entity::Entity, Game};

pub(crate) fn system(game: &mut Game) {
    let mut collisions = Vec::new();

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
                    if Rect::new(pos.x + vel.x / 10, pos.y + vel.y / 10, size.w, size.h)
                        .has_intersection(Rect::new(
                            other_pos.x,
                            other_pos.y,
                            other_size.w,
                            other_size.h,
                        ))
                    {
                        collisions.push(index);
                        break;
                    }
                }
            }
        }
    }

    for index in collisions {
        game.entities[index].vel.as_mut().unwrap().x = 0;
        game.entities[index].vel.as_mut().unwrap().y = 0;
    }

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
