use bevy::prelude::*;

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name(String::from("Alex"))));
    commands.spawn((Person, Name(String::from("John"))));
    commands.spawn((Person, Name(String::from("Mark"))));
}

fn greet_people(query: Query<&Name, With<Person>>) {
    for name in &query {
        println!("Hello, {}!", name.0);
    }
}

fn update_people(mut query: Query<&mut Name, With<Person>>) {
    for mut name in &mut query {
        if name.0 == "John" {
            name.0 = String::from("Marston");
            break;
        }
    }
}

fn hello_world() {
    println!("Hello, World!");
}

fn main() {
    App::new()
        .add_systems(Startup, add_people)
        .add_systems(Update, (hello_world, (update_people, greet_people).chain()))
        .run();
}
