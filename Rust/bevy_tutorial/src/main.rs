use std::fmt::Display;
use std::ops::Add;
use std::ops::AddAssign;

use bevy::prelude::*;
use bevy::window;

const WINDOW_WIDTH: f32 = 1200.0;
const WINDOW_HEIGHT: f32 = 720.0;

#[derive(Component)]
struct Position {
    x: f32,
    y: f32,
}

impl Add<&Velocity> for Position {
    type Output = Position;

    fn add(self, rhs: &Velocity) -> Self::Output {
        Position {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign<&Velocity> for Position {
    fn add_assign(&mut self, rhs: &Velocity) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Component)]
struct Velocity {
    x: f32,
    y: f32,
}

#[derive(Bundle)]
struct Object {
    position: Position,
    velocity: Velocity,
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resizable: true,
                        focused: true,
                        resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_systems(Startup, setup)
        .add_systems(Update, print_input)
        .add_systems(Update, update_position)
        .add_systems(Update, window::close_on_esc)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(SpriteBundle {
        texture: asset_server.load("res/player.png"),
        sprite: Sprite {
            color: Color::Rgba {
                red: 0.0,
                green: 1.0,
                blue: 0.0,
                alpha: 1.0,
            },
            flip_x: true,
            ..default()
        },
        ..default()
    });

    commands.spawn(Object {
        position: Position { x: 0.0, y: 0.0 },
        velocity: Velocity { x: 0.1, y: 0.1 },
    });
}

fn print_input(keyboard_input: Res<ButtonInput<KeyCode>>) {
    if keyboard_input.just_pressed(KeyCode::KeyA) {
        println!("Hello, World!");
    }
}

fn update_position(mut query: Query<(&mut Position, &Velocity)>) {
    for (mut position, velocity) in query.iter_mut() {
        *position += velocity;
        println!("{}", *position);
    }
}
