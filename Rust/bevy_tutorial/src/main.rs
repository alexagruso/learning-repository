use std::fmt::Display;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Mul;
use std::ops::MulAssign;

use bevy::prelude::*;
use bevy::window;

const WINDOW_WIDTH: f32 = 1200.0;
const WINDOW_HEIGHT: f32 = 720.0;

const MOVEMENT_SPEED: f32 = 5.0;
const SPRINT_MULTIPLIER: f32 = 2.5;

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

#[derive(Component, Clone)]
struct Velocity {
    x: f32,
    y: f32,
}

impl Mul<f32> for Velocity {
    type Output = Velocity;

    fn mul(self, rhs: f32) -> Self::Output {
        Velocity {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl MulAssign<f32> for Velocity {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl Display for Velocity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Bundle)]
struct Object {
    position: Position,
    velocity: Velocity,

    sprite: SpriteBundle,
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
        .add_systems(Update, (handle_input, update_position))
        .add_systems(Update, window::close_on_esc)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(Object {
        position: Position { x: 0.0, y: 0.0 },
        velocity: Velocity { x: 0.1, y: 0.1 },
        sprite: SpriteBundle {
            texture: asset_server.load("res/player.png"),
            sprite: Sprite {
                color: Color::Rgba {
                    red: 0.0,
                    green: 1.0,
                    blue: 1.0,
                    alpha: 1.0,
                },
                ..default()
            },
            transform: Transform {
                scale: Vec3 {
                    x: 5.0,
                    y: 5.0,
                    z: 5.0,
                },
                rotation: Quat::from_vec4(Vec4::from_array([0.0, 0.0, 0.25, 0.0])),
                ..default()
            },
            ..default()
        },
    });
}

fn handle_input(keyboard_input: Res<ButtonInput<KeyCode>>, mut query: Query<&mut Velocity>) {
    let mut new_velocity = Velocity { x: 0.0, y: 0.0 };

    if keyboard_input.pressed(KeyCode::KeyW) {
        new_velocity.y += MOVEMENT_SPEED;
    }

    if keyboard_input.pressed(KeyCode::KeyA) {
        new_velocity.x += -MOVEMENT_SPEED;
    }

    if keyboard_input.pressed(KeyCode::KeyS) {
        new_velocity.y += -MOVEMENT_SPEED;
    }

    if keyboard_input.pressed(KeyCode::KeyD) {
        new_velocity.x += MOVEMENT_SPEED;
    }

    if keyboard_input.pressed(KeyCode::ShiftLeft) {
        new_velocity *= SPRINT_MULTIPLIER;
    }

    for mut velocity in query.iter_mut() {
        *velocity = new_velocity.clone();
    }
}

fn update_position(mut query: Query<(&mut Position, &Velocity, &mut Transform, &mut Sprite)>) {
    for (mut position, velocity, mut transform, mut sprite) in query.iter_mut() {
        if velocity.x < 0.0 {
            sprite.flip_x = true;
        } else if velocity.x > 0.0 {
            sprite.flip_x = false;
        }

        *position += velocity;

        transform.translation = Vec3 {
            x: position.x,
            y: position.y,
            ..default()
        };
    }
}
