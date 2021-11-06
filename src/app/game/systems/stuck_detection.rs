use crate::app::game::{
    components::{Health, Vel},
    entity::Entity,
    Game,
};

use super::system_utils::get_collisions;

/// Detect stuck entities and unstuck them by changing pos
pub(crate) fn system(game: &mut Game) {
    let mut stuck_entities = Vec::new();

    for (index, entity) in game.entities.iter().enumerate() {
        if let Entity {
            pos: Some(current_pos),
            size: Some(current_size),
            vel: Some(_),
            physics: Some(_),
            health,
            ..
        } = entity
        {
            if *health == Some(Health::Dead) {
                continue;
            }
            let is_stuck = !get_collisions(&game.entities, index, *current_pos, *current_size).is_empty();
            if is_stuck {
                let mut offset = 1.0;
                let new_pos = loop {
                    let free_pos = [(1.0, 0.0), (-1.0, 0.0), (0.0, 1.0), (0.0, -1.0)]
                        .iter()
                        .map(|(x, y)| Vel { x: *x, y: *y })
                        .map(|vel| vel * offset)
                        .map(|vel| *current_pos + vel)
                        .find(|pos| get_collisions(&game.entities, index, *pos, *current_size).is_empty());

                    if let Some(pos) = free_pos {
                        break pos;
                    }

                    offset += 1.0;
                    if offset > 100.0 {
                        break *current_pos;
                    }
                };
                stuck_entities.push((index, new_pos));
            }
        }
    }

    for (index, pos) in stuck_entities {
        let entity = &mut game.entities[index];
        entity.pos = Some(pos);
    }
}
