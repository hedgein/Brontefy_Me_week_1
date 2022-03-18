#![allow(unused_parens)]
use bevy::sprite::collide_aabb::{collide, Collision};
use bevy::{prelude::*, reflect::TypeRegistry, utils::Duration};
use std::fs;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
struct Movement {
    location: Vec3,
    velocity: Vec3,
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
struct Background {
    dream: bool,
    real: bool,
}

#[derive(Component)]
enum Collider {
    Solid,
    Area,
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
struct Bed {
    id: u8,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
//        .register_type::<Movement>()
 //       .register_type::<Background>()
  //      .register_type::<Bed>()
        .add_startup_system(initial_real_setup.exclusive_system())
        .add_startup_system(real_texture_load.label("initial"))
        .add_startup_system(real_world_load.before("initial"))
        //        .add_system(keyboard_input)
        //        .add_system(player_movement)
        //        .add_system(switch_system)
        .run();
}
fn real_world_load( server: Res<AssetServer>, mut scene_spawner: ResMut<SceneSpawner>) {
    let scene_handle: Handle<DynamicScene> = server.load("scenes/real_scene.scn.ron");
    scene_spawner.spawn_dynamic(scene_handle);
    server.watch_for_changes().unwrap();
    println!("real world loaded");
    
}
fn real_texture_load(
    mut q: QuerySet<(
        QueryState<&mut Handle<Image>, With<Bed>>,
        QueryState<&mut Handle<Image>, With<Background>>,
        QueryState<&mut Handle<Image>, With<Movement>>,
    )>,
    server: Res<AssetServer>,
) {
    //    for mut bed_texture in q.q0().iter_mut() {
    //        let bed_handle = server.load("bed.png");
    //        *bed_texture = bed_handle;
    //        println!("bed loaded~");
    //   }

    let mut bed_query = q.q0();
    let mut bed_texture = bed_query.single_mut();
    let bed_handle = server.load("bed.png");
    *bed_texture = bed_handle;

    for mut bg_texture in q.q1().iter_mut() {
        let bg_handle = server.load("real.png");
        *bg_texture = bg_handle;
    }

    for mut player_texture in q.q2().iter_mut() {
        let player_handle = server.load("zeldo.png");
        *player_texture = player_handle;
    }
}

fn initial_real_setup(world: &mut World) {
    let mut real_scene_world = World::new();

    //setup camera
    real_scene_world
        .spawn()
        .insert_bundle(OrthographicCameraBundle::new_2d());
    //spawn real world textures
    real_scene_world
        .spawn()
        .insert_bundle(SpriteBundle {
            ..Default::default()
        })
        .insert(Background {
            real: true,
            dream: false,
        });

    real_scene_world
        .spawn()
        .insert_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(500.0, 0.0, 0.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert( Bed { id: 1 } );
    //  .insert(Collider::Area);

    //spawn player texture
    real_scene_world
        .spawn()
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(256.0, 144.0) / 3.5),
                ..Default::default()
            },
            //        transform: Transform::from_xyz(0.0, 0.0, 0.0),
            //        global_transform: GlobalTransform::from_xyz(0.0, 0.0, 0.0),
            //        visibility: Visibility { is_visible: true },
            ..Default::default()
        })
        .insert(Movement {
            location: Vec3::ZERO,
            velocity: Vec3::ZERO,
        });
    let real_type_registry = world.get_resource::<TypeRegistry>().unwrap();
    let real_scene = DynamicScene::from_world(&real_scene_world, real_type_registry);

    fs::write(
        "assets/scenes/real_scene.scn.ron",
        real_scene.serialize_ron(real_type_registry).unwrap(),
    );
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
    asset_server: Res<AssetServer>,
    mut scene_spawner: ResMut<SceneSpawner>,
    keys: Res<Input<KeyCode>>,
    player_query: Query<(&Transform, &Sprite), With<Movement>>,
    mut bg_query: Query<(&mut Handle<Image>, &mut Background)>,
    collider_query: Query<(Entity, &Collider, &Transform, &Sprite)>,
) {
    let (mut bg_texture, bg_background) = bg_query.single_mut();
    //let (bed, bed_transform) = bed_query.single();
    let (player_transform, player_sprite) = player_query.single();
    //    let bed_size = bed_transform.scale.truncate();

    for (_collider_entity, collider, transform, _sprite) in collider_query.iter() {
        let collision = collide(
            player_transform.translation,
            player_sprite
                .custom_size
                .unwrap_or(Vec2::new(256.0, 144.0) / 3.5),
            transform.translation,
            Vec2::new(100.0, 200.0),
        );

        if let Some(_collision) = collision {
            //area collision into bed
            if let Collider::Area = *collider {
                //switch to dream
                let dream_handle = asset_server.load("something here");
                scene_spawner.spawn_dynamic(dream_handle);
                //?watch for changes to assets?
            }
        }
    }

    if keys.just_pressed(KeyCode::R) {
        let real_handle = asset_server.load("FIX HERE");
        scene_spawner.spawn_dynamic(real_handle);
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
