use crate::{
    app::game::{entity::Entity, Game},
    draw::SCREEN_WIDTH,
};

/// Add velocity to position
pub(crate) fn system(game: &mut Game) {
    for entity in &mut game.entities {
        if let Entity {
            pos: Some(pos),
            vel: Some(vel),
            ..
        } = entity
        {
            *pos += *vel;

            if pos.x < 0.0 {
                pos.x += SCREEN_WIDTH as f32;
            }
            if pos.x > SCREEN_WIDTH as f32 {
                pos.x -= SCREEN_WIDTH as f32;
            }
        }
    }
}
