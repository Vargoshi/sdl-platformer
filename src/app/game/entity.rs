use parry2d::{
    math::{Real, Vector},
    shape::Cuboid,
};

use super::components;

pub(crate) struct Entity {
    pub pos: Option<Vector<Real>>,
    pub shape: Option<Cuboid>,
    pub vel: Option<Vector<Real>>,
    pub physics: Option<components::Physics>,
    pub health: Option<components::Health>,
    pub player: Option<components::Player>,
    pub enemy: Option<components::Enemy>,
    pub draw: Option<components::Draw>,
}
