use crate::{
    app::game::{entity::Entity, Game},
    draw::SCREEN_WIDTH,
};

/// Add velocity to position
pub(crate) fn system(game: &mut Game) {
    for entity in &mut game.entities {
        if let Entity {
            pos: Some(pos),
            size: Some(size),
            vel: Some(vel),
            ..
        } = entity
        {
            *pos += *vel;

            if pos.x + (size.w as i32 / 2) < 0 {
                pos.x += SCREEN_WIDTH as i32;
            }
            if pos.x + (size.w as i32 / 2) > SCREEN_WIDTH as i32 {
                pos.x -= SCREEN_WIDTH as i32;
            }
        }
    }
}
