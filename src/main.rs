mod todo;
mod todo_display;
mod todo_input;
use bevy::prelude::*;
use todo::*;
use todo_display::TodoDisplayPlugin;
use todo_input::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(TodoPlugin)
        .add_plugins(TodoInputPlugin)
        .add_plugins(TodoDisplayPlugin)
        .run();
}
