use sdl2::rect::Rect;

use crate::{
    app::game::{
        components::{Pos, Size, Vel},
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
) -> (Vel, bool, bool) {
    let mut vel = vel;

    let mut has_col_x = false;
    let mut has_col_y = false;

    while vel.x != 0 {
        if !has_collision(
            entities,
            skip_index,
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
            skip_index,
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

pub(crate) fn has_collision(entities: &[Entity], skip_index: usize, pos: Pos, size: Size) -> bool {
    entities
        .iter()
        .enumerate()
        .filter(|(index, _)| *index != skip_index)
        .filter_map(|(_, entity)| {
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
            ) || Rect::has_intersection(
                &Rect::new(pos.x + SCREEN_WIDTH as i32, pos.y, size.w, size.h),
                Rect::new(other_pos.x, other_pos.y, other_size.w, other_size.h),
            ) || Rect::has_intersection(
                &Rect::new(pos.x - SCREEN_WIDTH as i32, pos.y, size.w, size.h),
                Rect::new(other_pos.x, other_pos.y, other_size.w, other_size.h),
            )
        })
}
