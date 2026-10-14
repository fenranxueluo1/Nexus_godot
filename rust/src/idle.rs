use godot::prelude::Vector2;

use crate::player_state::PlayerState;
use crate::run::Run;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Idle;

impl Idle {
    pub(crate) fn process(&self, direction: Vector2) -> Option<PlayerState> {
        (direction.x != 0.0).then_some(PlayerState::Run(Run))
    }

    pub(crate) fn apply_physics(&self, velocity: &mut Vector2) {
        velocity.x = 0.0;
    }
}
