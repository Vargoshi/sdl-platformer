use crate::app::game::{
    components::{Dir, Health},
    entity::Entity,
    Game,
};

pub(crate) fn system(game: &mut Game) {
    for entity in &mut game.entities {
        if let Entity {
            vel: Some(vel),
            enemy: Some(enemy),
            health: Some(health),
            ..
        } = entity
        {
            if health == &Health::Alive {
                if enemy.dir == Dir::Left && vel.x > -20 {
                    vel.x -= 5;
                }
                if enemy.dir == Dir::Right && vel.x < 20 {
                    vel.x += 5;
                }
            }
        }
    }
}
