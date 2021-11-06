use std::collections::HashSet;

use sdl2::rect::Rect;

use crate::{
    app::game::{
        components::{Health, Pos, Size, Vel},
        entity::Entity,
    },
    draw::SCREEN_WIDTH,
};

/// Tries to move entity with `pos` and `size` by `vel`.
/// Returns maximum vel before collision with any other entity in `entities`
/// and whether there was collision in x axis and y axis.
/// If there was no collision at all returns the given `vel`.
pub(crate) fn move_entity(
    entities: &[Entity],
    skip_index: usize,
    pos: Pos,
    size: Size,
    vel: Vel,
) -> (Vel, bool, bool, HashSet<usize>) {
    let mut vel = vel;

    let mut has_col_x = false;
    let mut has_col_y = false;
    let mut contacts = HashSet::new();

    loop {
        let collisions = get_collisions(entities, skip_index, pos + Vel { x: vel.x, y: 0.0 }, size);
        if collisions.is_empty() {
            break;
        }
        contacts.extend(collisions);
        has_col_x = true;
        vel.x -= vel.x.signum();
        if vel.x.abs() < 1.0 {
            break;
        }
    }

    loop {
        let collisions = get_collisions(entities, skip_index, pos + vel, size);
        if collisions.is_empty() {
            break;
        }
        contacts.extend(collisions);
        has_col_y = true;
        vel.y -= vel.y.signum();
        if vel.y.abs() < 1.0 {
            break;
        }
    }

    (vel, has_col_x, has_col_y, contacts)
}

pub(crate) fn get_collisions(
    entities: &[Entity],
    skip_index: usize,
    pos: Pos,
    size: Size,
) -> HashSet<usize> {
    entities
        .iter()
        .enumerate()
        .filter(|(index, _entity)| *index != skip_index)
        .filter_map(|(index, entity)| {
            if let Entity {
                pos: Some(other_pos),
                size: Some(other_size),
                collision: Some(_),
                health: Some(Health::Alive) | None,
                ..
            } = entity
            {
                Some((index, other_pos, other_size))
            } else {
                None
            }
        })
        .filter(|(_index, other_pos, other_size)| {
            Rect::has_intersection(
                &Rect::new(pos.x, pos.y, size.w, size.h),
                Rect::new(other_pos.x, other_pos.y, other_size.w, other_size.h),
            ) || Rect::has_intersection(
                &Rect::new(pos.x + SCREEN_WIDTH as i32, pos.y, size.w, size.h),
                Rect::new(other_pos.x, other_pos.y, other_size.w, other_size.h),
            ) || Rect::has_intersection(
                &Rect::new(pos.x - SCREEN_WIDTH as i32, pos.y, size.w, size.h),
                Rect::new(other_pos.x, other_pos.y, other_size.w, other_size.h),
            )
        })
        .map(|(index, ..)| index)
        .collect()
}
