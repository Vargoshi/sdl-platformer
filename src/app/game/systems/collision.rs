use itertools::Itertools;
use parry2d::{
    math::{Isometry, Vector},
    query::time_of_impact,
};

use crate::app::game::{entity::Entity, Game};

/// Change velocity when collisions are detected
pub(crate) fn system(game: &mut Game) {
    let mut collisions = Vec::new();

    for entity in &mut game.entities {
        if let Entity {
            physics: Some(physics),
            ..
        } = entity
        {
            physics.on_floor = false;
            physics.on_left_wall = false;
            physics.on_right_wall = false;
            physics.contacts.clear();
        }
    }

    let ent_iter = game.entities.iter().enumerate().filter_map(|(idx, ent)| {
        if let Entity {
            pos: Some(pos),
            shape: Some(shape),
            vel,
            physics: Some(physics),
            ..
        } = ent
        {
            Some((idx, (pos, shape, vel, physics)))
        } else {
            None
        }
    });

    let ent_pair_iter = Itertools::cartesian_product(ent_iter.clone(), ent_iter.clone())
        .filter(|((idx1, _ent1), (idx2, _ent2))| idx1 < idx2);

    for ((idx1, ent1), (idx2, ent2)) in ent_pair_iter {
        let (pos1, shape1, vel1, _physics1) = ent1;
        let (pos2, shape2, vel2, _physics2) = ent2;

        if vel1.is_none() && vel2.is_none() {
            continue;
        }

        let maybe_impact = time_of_impact(
            &Isometry::new(*pos1, 0.0),
            &match vel1 {
                Some(vel) => *vel,
                None => Vector::zeros(),
            },
            shape1,
            &Isometry::new(*pos2, 0.0),
            &match vel2 {
                Some(vel) => *vel,
                None => Vector::zeros(),
            },
            shape2,
            1.0,
        )
        .unwrap();

        if let Some(impact) = maybe_impact {
            collisions.push((idx1, idx2, impact));
        }
    }

    for (idx1, idx2, impact) in collisions {
        if let Entity {
            vel: Some(vel),
            physics: Some(physics),
            ..
        } = &mut game.entities[idx1]
        {
            if impact.toi > 0.01 {
                *vel *= impact.toi * 0.99;
            } else {
                *vel = Vector::zeros();
            }

            if impact.normal1.dot(&Vector::new(0.0, 1.0)) > 0.5 {
                physics.on_floor = true;
            }
            if impact.normal1.dot(&Vector::new(1.0, 0.0)) > 0.5 {
                physics.on_right_wall = true;
            }
            if impact.normal1.dot(&Vector::new(-1.0, 0.0)) < 0.5 {
                physics.on_left_wall = true;
            }

            physics.contacts.insert(idx2);
        }

        if let Entity {
            vel: Some(vel),
            physics: Some(physics),
            ..
        } = &mut game.entities[idx2]
        {
            if impact.toi > 0.01 {
                *vel *= impact.toi * 0.99;
            } else {
                *vel = Vector::zeros();
            }

            if impact.normal2.dot(&Vector::new(0.0, 1.0)) > 0.5 {
                physics.on_floor = true;
            }
            if impact.normal2.dot(&Vector::new(1.0, 0.0)) > 0.5 {
                physics.on_right_wall = true;
            }
            if impact.normal2.dot(&Vector::new(-1.0, 0.0)) < 0.5 {
                physics.on_left_wall = true;
            }

            physics.contacts.insert(idx1);
        }
    }
}
