use super::components;

pub(crate) struct Entity {
    pub pos: Option<components::Pos>,
    pub size: Option<components::Size>,
    pub vel: Option<components::Vel>,
    pub collision: Option<components::Collision>,
    pub physics: Option<components::Physics>,
    pub health: Option<components::Health>,
    pub player: Option<components::Player>,
    pub enemy: Option<components::Enemy>,
}
