use bevy::log::{self, info};
use bevy::prelude::*;
use std::io::{Read, Write};

pub struct TodoPlugin;
impl Plugin for TodoPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Todos(vec![]));
        app.add_systems(Startup, init_todos);
        app.add_systems(Update, (add_todos, remove_todos, update_todos));
    }
}

const TODO_PATH: &str = "todo.txt";

#[derive(Resource)]
pub struct Todos(pub Vec<Todo>);
impl Todos {
    fn load_file(&mut self, mut file: std::fs::File) {
        let mut content = String::new();
        file.read_to_string(&mut content)
            .expect("Reading the file to string");
        let mut todos: Vec<Todo> = vec![];
        for line in content.lines() {
            todos.push(Todo::from(line.trim_end()))
        }
        self.0 = todos;
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    fn save(&self) {
        let mut file = match std::fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(TODO_PATH)
        {
            Ok(f) => f,
            Err(_) => panic!("Failed to open TODO file"),
        };
        for todo in &self.0 {
            let mut todo_str: String = todo.to_string();
            todo_str.push_str("\n");
            match file.write(todo_str.as_str().as_bytes()) {
                Ok(_) => {}
                Err(_) => panic!("Failed to write string to file"),
            };
        }
    }

    fn add_todo(&mut self, todo: Todo) {
        self.0.push(todo);
        self.save();
    }

    fn remove_todo(&mut self, idx: usize) {
        if self.0.len() > 0 {
            self.0.remove(idx);
            self.save();
        }
    }

    fn update_todo(&mut self, idx: usize, todo: Todo) {
        self.0[idx] = todo;
        self.save();
    }
}

/// in file if not complete = `0;name`
/// in file if complete = `1;name`
#[derive(Component, Clone)]
pub struct Todo {
    pub completed: bool,
    pub name: String,
}

#[derive(Component)]
pub struct TAdd;
#[derive(Component)]
pub struct TUpdate(pub(crate) usize, pub(crate) Todo);
#[derive(Component)]
pub struct TRemove(pub(crate) usize);

impl Todo {
    pub fn new(completed: bool, name: String) -> Self {
        Self { completed, name }
    }
}

impl ToString for Todo {
    fn to_string(&self) -> String {
        format!("{};{}", if self.completed { 1 } else { 0 }, self.name)
    }
}

impl From<String> for Todo {
    fn from(s: String) -> Todo {
        assert!(s.len() >= 3);
        assert!(s.contains(";"));
        let completed: bool = s.chars().nth(0).unwrap() == '1';
        let name: String = s[s.find(";").unwrap() + 1..].to_string();
        Todo { name, completed }
    }
}
impl From<&str> for Todo {
    fn from(s: &str) -> Todo {
        assert!(s.len() >= 3);
        assert!(s.contains(";"));
        let completed: bool = s.chars().nth(0).unwrap() == '1';
        let name: String = s[s.find(";").unwrap() + 1..].to_string();
        Todo { name, completed }
    }
}

fn init_todos(mut todos: ResMut<Todos>) {
    info!("Initializing Todos...");
    if !std::fs::exists(TODO_PATH).unwrap_or(false) {
        log::debug!("Creating new TODO file");
        match std::fs::File::create_new(TODO_PATH) {
            Ok(_) => {}
            Err(_) => panic!("Failed to create TODO file"),
        }
        todos.0 = vec![];
    } else {
        log::debug!("Loading TODO file");
        let file = match std::fs::File::open(TODO_PATH) {
            Ok(f) => f,
            Err(_) => panic!("Failed to open TODO file"),
        };
        todos.load_file(file);
    }
}

fn add_todos(
    mut commands: Commands,
    mut todos: ResMut<Todos>,
    query: Query<(Entity, &Todo), With<TAdd>>,
) {
    for (e, todo) in &query {
        todos.add_todo(todo.clone());
        commands.entity(e).despawn();
    }
}
fn update_todos(
    mut commands: Commands,
    mut todos: ResMut<Todos>,
    query: Query<(Entity, &TUpdate)>,
) {
    for (e, TUpdate(idx, todo)) in &query {
        todos.update_todo(*idx, todo.clone());
        commands.entity(e).despawn();
    }
}
fn remove_todos(
    mut commands: Commands,
    mut todos: ResMut<Todos>,
    query: Query<(Entity, &TRemove)>,
) {
    for (e, TRemove(idx)) in &query {
        todos.remove_todo(*idx);
        commands.entity(e).despawn();
    }
}
