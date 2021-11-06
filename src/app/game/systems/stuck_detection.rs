use crate::app::game::{components::Vel, entity::Entity, Game};

use super::system_utils::has_collision;

/// Detect stuck entities and unstuck them by changing pos
pub(crate) fn system(game: &mut Game) {
    let mut stuck_entities = Vec::new();

    for (index, entity) in game.entities.iter().enumerate() {
        if let Entity {
            pos: Some(current_pos),
            size: Some(current_size),
            vel: Some(_),
            physics: Some(_),
            ..
        } = entity
        {
            let is_stuck = has_collision(&game.entities, index, *current_pos, *current_size);
            if is_stuck {
                let mut offset = 1.0;
                let new_pos = loop {
                    let free_pos = [(1.0, 0.0), (-1.0, 0.0), (0.0, 1.0), (0.0, -1.0)]
                        .iter()
                        .map(|(x, y)| Vel { x: *x, y: *y })
                        .map(|vel| vel * offset)
                        .map(|vel| *current_pos + vel)
                        .find(|pos| !has_collision(&game.entities, index, *pos, *current_size));

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
