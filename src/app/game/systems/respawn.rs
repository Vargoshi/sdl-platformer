use crate::{
    app::game::{components::Health, entity::Entity, Game},
    draw::{SCREEN_HEIGHT, SCREEN_WIDTH},
};

pub(crate) fn system(game: &mut Game) {
    for entity in &mut game.entities {
        if let Entity {
            health: Some(health @ Health::Dead),
            pos: Some(pos),
            vel: Some(vel),
            ..
        } = entity
        {
            if pos.y > SCREEN_HEIGHT as i32 {
                pos.x = (SCREEN_WIDTH as i32) / 4;
                pos.y = 0;
                vel.y = 0.0;
                *health = Health::Alive;
            }
        }
    }
}
