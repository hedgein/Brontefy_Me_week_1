#![allow(unused_parens)]

use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;

#[derive(Component)]
struct Movement {
    location: Vec3,
    velocity: Vec3,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(load_sprite)
        .add_system(keyboard_input)
        .add_system(player_movement)
        .run();
}

fn load_sprite(mut commands: Commands, server: Res<AssetServer>, windows: Res<Windows>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    let handle = server.load("zeldo.png");
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(256.0, 144.0) / 3.5),
                ..Default::default()
            },
            texture: handle,
            //        transform: Transform::from_xyz(0.0, 0.0, 0.0),
            //        global_transform: GlobalTransform::from_xyz(0.0, 0.0, 0.0),
            //        visibility: Visibility { is_visible: true },
            ..Default::default()
        })
        .insert(Movement {
            location: Vec3::ZERO,
            velocity: Vec3::ZERO,
        });

    let window = windows.get_primary().unwrap();
    println!("Width {}, Height {}", window.width(), window.height());
}

fn keyboard_input(
    mut player_query: Query<(&mut Movement)>,
    keys: Res<Input<KeyCode>>,
) {
    let mut movement = player_query.single_mut();
    movement.velocity = Vec3::ZERO;
    for key in keys.get_pressed() {
        movement.velocity += match key {
            KeyCode::W => Vec3::new(0.0, 1.0, 0.0),
            KeyCode::A => Vec3::new(-1.0, 0.0, 0.0),
            KeyCode::S => Vec3::new(0.0, -1.0, 0.0),
            KeyCode::D => Vec3::new(1.0, 0.0, 0.0),
            _ => Vec3::ZERO,
        };
    }
}

fn player_movement(mut player_query: Query<(&mut Transform, &mut Movement)>) {
    //update transform movement here
    let (mut transform, mut movement) = player_query.single_mut();
    if movement.velocity != Vec3::ZERO {
        let player_velocity = movement.velocity.normalize();
        movement.location += player_velocity;
    }
    transform.translation = movement.location;
}
