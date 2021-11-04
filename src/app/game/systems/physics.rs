use sdl2::rect::Rect;

use crate::app::game::{
    components::{Pos, Size, Vel},
    entity::Entity,
    Game,
};

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
            let (offset, col_x, col_y) = move_entity(&game.entities, *pos, *size, *vel);
            if col_x || col_y {
                collisions.push((index, offset, col_y && vel.y > 0));
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

/// Tries to move entity with `pos` and `size` by `vel`.
/// Returns maximum vel before collision with any other entity in `entities`
/// and whether there was collision in x axis and y axis.
/// If there was no collision at all returns the given `vel`.
fn move_entity(entities: &[Entity], pos: Pos, size: Size, vel: Vel) -> (Vel, bool, bool) {
    let mut vel = vel;

    let mut has_col_x = false;
    let mut has_col_y = false;

    while vel.x != 0 {
        if !has_collision(
            entities,
            Pos {
                x: pos.x + vel.x / 10,
                y: pos.y,
            },
            size,
        ) {
            break;
        }
        has_col_x = true;
        vel.x -= vel.x.signum();
    }

    while vel.y != 0 {
        if !has_collision(
            entities,
            Pos {
                x: pos.x + vel.x / 10,
                y: pos.y + vel.y / 10,
            },
            size,
        ) {
            break;
        }
        has_col_y = true;
        vel.y -= vel.y.signum();
    }

    (vel, has_col_x, has_col_y)
}

fn has_collision(entities: &[Entity], pos: Pos, size: Size) -> bool {
    entities
        .iter()
        .filter_map(|entity| {
            if let Entity {
                pos: Some(other_pos),
                size: Some(other_size),
                collision: Some(_),
                ..
            } = entity
            {
                Some((other_pos, other_size))
            } else {
                None
            }
        })
        .any(|(other_pos, other_size)| {
            Rect::has_intersection(
                &Rect::new(pos.x, pos.y, size.w, size.h),
                Rect::new(other_pos.x, other_pos.y, other_size.w, other_size.h),
            )
        })
}
