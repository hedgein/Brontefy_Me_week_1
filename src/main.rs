#![allow(unused_parens)]
use bevy::prelude::*;
use bevy::sprite::collide_aabb::{collide, Collision};

#[derive(Component)]
struct Movement {
    location: Vec3,
    velocity: Vec3,
}

#[derive(Component)]
struct Background {
    dream: Handle<Image>,
    real: Handle<Image>,
}

#[derive(Component)]
enum Collider {
    Solid,
    Area,
}

#[derive(Component)]
struct Bed {
    id: u8,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(load_sprite)
        .add_system(keyboard_input)
        .add_system(player_movement)
        .add_system(switch_system)
        .run();
}

fn load_sprite(mut commands: Commands, server: Res<AssetServer>) {
    //setup camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    //set handles for player and initial real world
    let handle = server.load("zeldo.png");
    let bg_handle = server.load("real.png");
    let bed_handle = server.load("bed.png");

    //spawn real world textures
    commands
        .spawn_bundle(SpriteBundle {
            texture: bg_handle,
            ..Default::default()
        })
        .insert(Background {
            real: server.load("real.png"),
            dream: server.load("dream.png"),
        });

    commands
        .spawn_bundle(SpriteBundle {
            texture: bed_handle,
            transform: Transform {
                translation: Vec3::new(500.0, 0.0, 0.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Bed { id: 1 })
        .insert(Collider::Area);

    //spawn player texture
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
}

fn keyboard_input(mut player_query: Query<(&mut Movement)>, keys: Res<Input<KeyCode>>) {
    //Single query for movement component in player entity
    let mut movement = player_query.single_mut();
    //Clear velocities to zero
    movement.velocity = Vec3::ZERO;
    //Match movement keys, set new velocity to movement component
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

fn switch_system(
    //bed_query: Query<(&Bed, &Transform)>,
    keys: Res<Input<KeyCode>>,
    player_query: Query<(&Transform, &Sprite), With<Movement>>,
    mut bg_query: Query<(&mut Handle<Image>, &mut Background)>,
    collider_query: Query<(Entity, &Collider, &Transform, &Sprite)>,
) {
    let (mut bg_texture, bg_background) = bg_query.single_mut();
    //let (bed, bed_transform) = bed_query.single();
    let (player_transform, player_sprite) = player_query.single();
//    let bed_size = bed_transform.scale.truncate();

    for (_collider_entity, collider, transform, _sprite) in collider_query.iter(){
        let collision = collide(
                player_transform.translation,
                player_sprite.custom_size.unwrap_or(Vec2::new(256.0, 144.0) /3.5),
                transform.translation,
                Vec2::new(100.0, 200.0),
            );

        if let Some(_collision) = collision {
            //area collision into bed
            if let Collider::Area = *collider {
            //switch to dream
               *bg_texture = bg_background.dream.clone(); 
            }
        }
    }

    if keys.just_pressed(KeyCode::R) {
        *bg_texture = bg_background.real.clone();
    }
}

fn player_movement(mut player_query: Query<(&mut Transform, &mut Movement), With<Movement>>) {
    //update transform movement here
    let (mut transform, mut movement) = player_query.single_mut();
    //Only normalize if there is movement
    if movement.velocity != Vec3::ZERO {
        //Normalize vector velocities in any direction
        let player_velocity = movement.velocity.normalize();
        //change or adjust movement speed here
        let speed_scale = 1.5;
        //change location according to velocity
        movement.location += player_velocity * speed_scale;
    }
    //Update translation according to movement.location
    transform.translation = movement.location;
}
