use crate::todo::*;
use bevy::log::info;
use bevy::{
    input::{
        ButtonState,
        keyboard::{Key, KeyboardInput},
    },
    prelude::*,
};

pub struct TodoInputPlugin;
impl Plugin for TodoInputPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SelectedTodo(None));
        app.insert_resource(TodoInputState {
            typing: false,
            text: String::new(),
        });
        app.add_systems(Update, (handle_keyboard, handle_input));
    }
}

#[derive(Resource)]
pub struct SelectedTodo(pub Option<usize>);

#[derive(Resource)]
pub struct TodoInputState {
    pub typing: bool,
    pub text: String,
}

fn handle_input(
    mut commands: Commands,
    keycode: Res<ButtonInput<KeyCode>>,
    todos: Res<Todos>,
    mut selected_todo: ResMut<SelectedTodo>,
    mut tis: ResMut<TodoInputState>,
) {
    if tis.typing {
        // dont accept any other keys while typing
        return;
    }
    if keycode.just_pressed(KeyCode::KeyA) {
        tis.typing = true;
        return;
    }
    if keycode.just_pressed(KeyCode::KeyD) {
        if let Some(idx) = selected_todo.0 {
            if idx >= todos.len() - 1 {
                if idx == 0 {
                    selected_todo.0 = None;
                } else {
                    selected_todo.0 = Some(todos.len() - 2); // -2 because len != index + remove
                }
                info!("Selected is now: {:?}", selected_todo.0);
            }
            commands.spawn(TRemove(idx));
        }
    }
    if keycode.just_pressed(KeyCode::ArrowDown) {
        if let Some(idx) = selected_todo.0 {
            if todos.len() < 1 {
                selected_todo.0 = None;
            } else if todos.len() <= idx + 1 {
                selected_todo.0 = Some(0);
            } else {
                selected_todo.0 = Some(idx + 1);
            }
        } else {
            if todos.len() < 1 {
                selected_todo.0 = None;
            } else {
                selected_todo.0 = Some(0);
            }
        }
    }
    if keycode.just_pressed(KeyCode::ArrowUp) {
        if let Some(idx) = selected_todo.0 {
            if todos.len() < 1 {
                selected_todo.0 = None;
            } else if idx == 0 {
                selected_todo.0 = Some(todos.len() - 1);
            } else {
                selected_todo.0 = Some(idx - 1);
            }
        } else {
            if todos.len() < 1 {
                selected_todo.0 = None;
            } else {
                selected_todo.0 = Some(todos.len() - 1);
            }
        }
    }
}

fn handle_keyboard(
    mut commands: Commands,
    mut evr_kbd: EventReader<KeyboardInput>,
    mut tis: ResMut<TodoInputState>,
) {
    if !tis.typing {
        // dont log keys when its not typing time
        evr_kbd.clear();
        return;
    }
    for ev in evr_kbd.read() {
        // We don't care about key releases, only key presses
        if ev.state == ButtonState::Released {
            continue;
        }
        match &ev.logical_key {
            // Handle pressing Enter to finish the input
            Key::Enter => {
                info!("Text input: {}", tis.text);
                commands.spawn((Todo::new(false, tis.text.clone()), TAdd));
                tis.text.clear();
                tis.typing = false;
            }
            // Handle pressing Backspace to delete last char
            Key::Backspace => {
                tis.text.pop();
            }
            Key::Space => {
                tis.text.push_str(" ");
            }
            // Handle key presses that produce text characters
            Key::Character(input) => {
                // Ignore any input that contains control (special) characters
                if input.chars().any(|c| c.is_control()) {
                    continue;
                }
                tis.text.push_str(&input);
            }
            _ => {}
        }
    }
}
