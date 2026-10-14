use godot::classes::{CharacterBody2D, ICharacterBody2D, Input, Label};
use godot::prelude::*;

use crate::player_state::{PlayerState, StateContext};

#[derive(GodotClass)]
#[class(base = CharacterBody2D)]
struct Player {
    #[export]
    move_speed: f32,
    direction: Vector2,
    gravity: f32,
    state: PlayerState,
    base: Base<CharacterBody2D>,
}

#[godot_api]
impl ICharacterBody2D for Player {
    fn init(base: Base<CharacterBody2D>) -> Self {
        Self {
            move_speed: 100.0,
            direction: Vector2::ZERO,
            gravity: 980.0,
            state: PlayerState::initial(),
            base,
        }
    }

    fn ready(&mut self) {
        self.update_debug_label();
    }

    fn process(&mut self, _delta: f64) {
        self.update_direction();

        let state = self.state;
        let next = state.process(&self.state_context());
        self.change_state(next);
    }

    fn physics_process(&mut self, delta: f64) {
        let state = self.state;
        let ctx = self.state_context();

        let mut velocity = self.base().get_velocity();
        velocity.y += self.gravity * delta as f32;
        state.apply_physics(&ctx, &mut velocity);

        let mut base = self.base_mut();
        base.set_velocity(velocity);
        base.move_and_slide();
    }
}

impl Player {
    fn state_context(&self) -> StateContext {
        StateContext {
            direction: self.direction,
            move_speed: self.move_speed,
        }
    }

    fn update_direction(&mut self) {
        let input = Input::singleton();
        self.direction = Vector2::new(
            input.get_axis("左", "右"),
            input.get_axis("上", "下"),
        );
    }

    fn change_state(&mut self, next: Option<PlayerState>) {
        let Some(next) = next else { return };
        if next == self.state {
            return;
        }

        self.state = next;
        self.update_debug_label();
    }

    fn update_debug_label(&mut self) {
        let Some(mut label) = self.base().try_get_node_as::<Label>("Label") else {
            return;
        };

        let text = self.state.name();
        label.set_text(text);
    }
}
