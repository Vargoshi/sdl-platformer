use sdl2::rect::Rect;

use crate::app::game::{components::Health, entity::Entity, Game};

pub(crate) fn system(game: &mut Game) {
    let mut damages = Vec::new();

    for (index, entity) in game.entities.iter().enumerate() {
        if let Entity {
            player: Some(_),
            health: Some(Health::Alive),
            pos: Some(pos),
            size: Some(size),
            ..
        } = entity
        {
            for other_entity in &game.entities {
                if let Entity {
                    enemy: Some(_),
                    pos: Some(other_pos),
                    size: Some(other_size),
                    ..
                } = other_entity
                {
                    if Rect::has_intersection(
                        &Rect::new(pos.x, pos.y, size.w, size.h),
                        Rect::new(other_pos.x, other_pos.y, other_size.w, other_size.h),
                    ) {
                        damages.push(index)
                    }
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
