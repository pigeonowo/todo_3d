use crate::{todo::Todos, todo_input::*};
use bevy::prelude::*;

pub struct TodoDisplayPlugin;
impl Plugin for TodoDisplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Update, display_selected);
        app.add_systems(Update, display_todos);
        // text input
    }
}

#[derive(Component)]
struct ControlText;
#[derive(Component)]
struct SelectedText;
#[derive(Component)]
struct TodoText;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    let control_text_font = TextFont {
        font_size: 12.0,
        ..default()
    };
    commands.spawn((
        Text::new("A - Add Todo after writing text\nDOWN - Select Todo Down\nUP - Select Todo Up\nD - Delete Selected TODO"),
        control_text_font,
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(12.0),
            left: Val::Px(12.0),
            ..default()
        },
        ControlText
    ));

    let selected_text_font = TextFont {
        font_size: 12.0,
        ..default()
    };
    commands.spawn((
        Text::default(),
        selected_text_font,
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(12.0),
            left: Val::Px(12.0),
            ..default()
        },
        SelectedText,
    ));
}

fn display_selected(
    mut selected_text: Single<&mut Text, With<SelectedText>>,
    selected_todo: Res<SelectedTodo>,
) {
    let mut txt = String::from("Selected: ");
    if let Some(idx) = selected_todo.0 {
        txt.push_str(&idx.to_string());
    } else {
        txt.push_str("None");
    }
    selected_text.0 = txt;
}

fn display_todos(
    mut commands: Commands,
    todos: Res<Todos>,
    selected_todo: Res<SelectedTodo>,
    todo_texts: Query<Entity, With<TodoText>>,
) {
    // remove old todos
    for e in &todo_texts {
        commands.entity(e).despawn();
    }
    // create new ones
    let todo_text_font = TextFont {
        font_size: 24.0,
        ..default()
    };
    let mut top_pos = 10.0;
    for (i, todo) in (&todos.0).iter().enumerate() {
        commands.spawn((
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(top_pos),
                left: Val::Px(275.0),
                ..default()
            },
            Text::new(&todo.name),
            TextLayout::new_with_justify(JustifyText::Center),
            if let Some(x) = selected_todo.0 {
                if i == x {
                    TextColor::BLACK
                } else {
                    TextColor::WHITE
                }
            } else {
                TextColor::WHITE
            },
            todo_text_font.clone(),
            TodoText,
        ));
        top_pos += 50.0;
    }
}
