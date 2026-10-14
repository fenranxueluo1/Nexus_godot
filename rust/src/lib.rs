mod idle;
mod player;
mod player_state;
mod run;

use godot::prelude::*;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}
