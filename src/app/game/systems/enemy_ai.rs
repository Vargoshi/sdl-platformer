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
            physics: Some(physics),
            ..
        } = entity
        {
            if health == &Health::Alive {
                if enemy.dir == Dir::Left && vel.x > -1.0 {
                    vel.x -= 0.5;
                }
                if enemy.dir == Dir::Right && vel.x < 1.0 {
                    vel.x += 0.5;
                }

                if enemy.jumping && physics.on_floor {
                    vel.y -= 4.0;
                }
            }

            if physics.on_left_wall {
                enemy.dir = Dir::Right;
            }
            if physics.on_right_wall {
                enemy.dir = Dir::Left;
            }
        }
    }
}
