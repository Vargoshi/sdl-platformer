use std::collections::HashSet;

use itertools::Itertools;
use parry2d::{
    math::{Isometry, Real, Vector},
    query::{contact, time_of_impact, Contact, TOI},
    shape::Shape,
};

use crate::app::game::{entity::Entity, Game};

/// Change velocity when collisions are detected
pub(crate) fn system(game: &mut Game) {
    let mut collisions = Vec::new();

    // Reset physics state
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

    fn toi(
        pos1: &Vector<Real>,
        vel1: &Option<Vector<Real>>,
        shape1: &dyn Shape,
        pos2: &Vector<Real>,
        vel2: &Option<Vector<Real>>,
        shape2: &dyn Shape,
    ) -> Option<TOI> {
        time_of_impact(
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
        .unwrap()
    }

    // Find all collisions
    for (self_idx, self_ent) in game.entities.iter().enumerate() {
        if let Entity {
            pos: Some(self_pos),
            shape: Some(self_shape),
            vel: Some(self_vel),
            physics: Some(_),
            ..
        } = self_ent
        {
            let mut shortest_impact: Option<TOI> = None;

            for (other_idx, other_ent) in game
                .entities
                .iter()
                .enumerate()
                .filter(|(idx, _)| *idx != self_idx)
            {
                if let Entity {
                    pos: Some(other_pos),
                    shape: Some(other_shape),
                    vel: other_vel,
                    physics: Some(_),
                    ..
                } = other_ent
                {
                    if let Some(impact) = toi(
                        self_pos,
                        &Some(*self_vel),
                        self_shape,
                        other_pos,
                        other_vel,
                        other_shape,
                    ) {
                        if impact.toi < shortest_impact.map(|impact| impact.toi).unwrap_or(1.0) {
                            shortest_impact = Some(impact);
                        }
                    }

                    if let Some(impact) = toi(
                        self_pos,
                        &Some(*self_vel),
                        self_shape,
                        other_pos,
                        &None,
                        other_shape,
                    ) {
                        if impact.toi < shortest_impact.map(|impact| impact.toi).unwrap_or(1.0) {
                            shortest_impact = Some(impact);
                        }
                    }
                }
            }

            if let Some(impact) = shortest_impact {
                let remaining_toi = 1.0 - impact.toi;
                let before_impact_vel = self_vel * impact.toi;
                let after_impact_vel = self_vel * remaining_toi;
                let normal_vel = *impact.normal2 * after_impact_vel.dot(&impact.normal2);
                let tangent_vel = after_impact_vel - normal_vel;

                let mut shortest_impact: Option<TOI> = None;

                for (other_idx, other_ent) in game.entities.iter().enumerate() {
                    if other_idx == self_idx {
                        continue;
                    }
                    if let Entity {
                        pos: Some(other_pos),
                        shape: Some(other_shape),
                        vel: other_vel,
                        physics: Some(_),
                        ..
                    } = other_ent
                    {
                        if let Some(impact) = toi(
                            &(self_pos + before_impact_vel),
                            &Some(tangent_vel),
                            self_shape,
                            other_pos,
                            other_vel,
                            other_shape,
                        ) {
                            if impact.toi < shortest_impact.map(|impact| impact.toi).unwrap_or(1.0)
                            {
                                shortest_impact = Some(impact);
                            }
                        }

                        if let Some(impact) = toi(
                            &(self_pos + before_impact_vel),
                            &Some(tangent_vel),
                            self_shape,
                            other_pos,
                            &None,
                            other_shape,
                        ) {
                            if impact.toi < shortest_impact.map(|impact| impact.toi).unwrap_or(1.0)
                            {
                                shortest_impact = Some(impact);
                            }
                        }
                    }
                }

                if let Some(impact2) = shortest_impact {
                    let toi = remaining_toi * impact2.toi;
                    let before_impact2_vel = tangent_vel * impact2.toi;
                    let vel = before_impact_vel + before_impact2_vel;
                    println!("2 {}", vel);
                    collisions.push((self_idx, vel, HashSet::<usize>::new(), false, false, false));
                } else {
                    let vel = before_impact_vel + tangent_vel;
                    println!("1 {}", vel);
                    collisions.push((self_idx, vel, HashSet::<usize>::new(), false, false, false));
                }
            }
        }
    }

    //dbg!(&collisions);
    // Mutate colliding entities
    for (self_idx, new_vel, other_indices, on_floor, on_left_wall, on_right_wall) in collisions {
        if let Entity { vel: Some(vel), .. } = &mut game.entities[self_idx] {
            *vel = Vector::new(new_vel.x.floor(), new_vel.y.floor());
        }
        if let Entity {
            physics: Some(physics),
            ..
        } = &mut game.entities[self_idx]
        {
            physics.on_floor = on_floor;
            physics.on_right_wall = on_right_wall;
            physics.on_left_wall = on_left_wall;
            physics.contacts.extend(other_indices);
        }
    }
}
