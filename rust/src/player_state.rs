use godot::prelude::Vector2;

use crate::idle::Idle;
use crate::run::Run;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum PlayerState {
    Idle(Idle),
    Run(Run),
}

pub(crate) struct StateContext {
    pub direction: Vector2,
    pub move_speed: f32,
}

impl PlayerState {
    pub(crate) fn initial() -> Self {
        Self::Idle(Idle)
    }

    pub(crate) fn name(self) -> &'static str {
        match self {
            Self::Idle(_) => "Idle",
            Self::Run(_) => "Run",
        }
    }

    pub(crate) fn process(self, ctx: &StateContext) -> Option<Self> {
        match self {
            Self::Idle(state) => state.process(ctx.direction),
            Self::Run(state) => state.process(ctx.direction),
        }
    }

    pub(crate) fn apply_physics(self, ctx: &StateContext, velocity: &mut Vector2) {
        match self {
            Self::Idle(state) => state.apply_physics(velocity),
            Self::Run(state) => state.apply_physics(velocity, ctx.direction, ctx.move_speed),
        }
    }
}
