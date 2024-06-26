use std::fmt::Display;

use bevy::prelude::*;
use bevy::window;
use bevy_tutorial::position::Position;

const WINDOW_WIDTH: f32 = 1200.0;
const WINDOW_HEIGHT: f32 = 720.0;

const MOVEMENT_SPEED: f32 = 5.0;
const SPRINT_MULTIPLIER: f32 = 2.5;

#[derive(Component)]
struct Projectile {
    position: Position,
    velocity: Velocity,
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
                    x: 2.0,
                    y: 2.0,
                    ..default()
                },
                rotation: Quat::from_vec4(Vec4::from_array([0.0, 0.0, 0.25, 0.0])),
                ..default()
            },
            ..default()
        },
    });
}

fn in_bounds(position: &Position, velocity: &Velocity) -> bool {
    let x_bound = WINDOW_WIDTH / 2.0;
    let y_bound = WINDOW_HEIGHT / 2.0;

    let position = position.clone() + velocity;

    if position.x < -x_bound
        || position.x > x_bound
        || position.y < -y_bound
        || position.y > y_bound
    {
        false
    } else {
        true
    }
}

fn in_bounds_x(x_position: f32) -> bool {
    let x_bound = WINDOW_WIDTH / 2.0;

    if x_position < -x_bound || x_position > x_bound {
        false
    } else {
        true
    }
}

fn in_bounds_y(y_position: f32) -> bool {
    let y_bound = WINDOW_HEIGHT / 2.0;

    if y_position < -y_bound || y_position > y_bound {
        false
    } else {
        true
    }
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

        if in_bounds_x(position.x + velocity.x) {
            position.x += velocity.x
        }

        if in_bounds_y(position.y + velocity.y) {
            position.y += velocity.y
        }

        transform.translation = Vec3 {
            x: position.x,
            y: position.y,
            ..default()
        };
    }
}

fn spawn_projectile(mouse_input: Res<ButtonInput<MouseButton>>, mut commands: Commands) {
    if mouse_input.just_pressed(MouseButton::Left) {
        commands.spawn()
    }
}
