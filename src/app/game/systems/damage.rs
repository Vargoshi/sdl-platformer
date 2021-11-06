use std::collections::HashSet;

use crate::app::game::{components::Health, entity::Entity, Game};

pub(crate) fn system(game: &mut Game) {
    let mut damages = HashSet::new();

    for (index, entity) in game.entities.iter().enumerate() {
        if let Entity {
            player: Some(_),
            health: Some(Health::Alive),
            physics: Some(physics),
            ..
        } = entity
        {
            for contact in &physics.contacts {
                let other_entity = &game.entities[*contact];
                if let Entity { enemy: Some(_), .. } = other_entity {
                    damages.insert(index);
                }
            }
        }
    }

    for entity in &game.entities {
        if let Entity {
            enemy: Some(_),
            physics: Some(physics),
            ..
        } = entity
        {
            for contact in &physics.contacts {
                let other_entity = &game.entities[*contact];
                if let Entity {
                    player: Some(_),
                    health: Some(Health::Alive),
                    ..
                } = other_entity
                {
                    damages.insert(*contact);
                }
            }
        }
    }

    for index in damages {
        let entity = &mut game.entities[index];
        if let Some(vel) = &mut entity.vel {
            vel.x = 0.0;
            vel.y = -5.0;
        }
        entity.health = Some(Health::Dead);
    }
}
