use godot::prelude::Vector2;

use crate::idle::Idle;
use crate::player_state::PlayerState;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Run;

impl Run {
    pub(crate) fn process(&self, direction: Vector2) -> Option<PlayerState> {
        (direction.x == 0.0).then_some(PlayerState::Idle(Idle))
    }

    pub(crate) fn apply_physics(&self, velocity: &mut Vector2, direction: Vector2, move_speed: f32) {
        velocity.x = direction.x * move_speed;
    }
}
