use crate::app::game::{entity::Entity, Game};

/// Apply gravity and friction to velocity
pub(crate) fn system(game: &mut Game) {
    for entity in &mut game.entities {
        if let Entity {
            vel: Some(vel),
            physics: Some(physics),
            ..
        } = entity
        {
            vel.y += physics.gravity;
            vel.x *= (1.0 - physics.friction);
        }
    }
}
