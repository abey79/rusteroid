use crate::components::{Asteroid, Flame, LifeTime, Missile, RotationSpeed, Ship, Speed, Thruster};
use crate::events::{AsteroidKillEvent, AsteroidSpawnEvent};
use crate::line_sprite::{LineMaterial, LineSpriteBundleBuilder, Shape};
use crate::{Resolution, TIME_STEP};
use bevy::prelude::*;
use geo::Intersects;
use rand::Rng;
use std::f32::consts::PI;
use std::time::Duration;

pub fn ship_motion_system(
    mut q_parent: Query<(&mut Speed, &mut Transform, &Thruster, &Children, &Ship)>,
    mut q_child: Query<&mut Visibility, With<Flame>>,
) {
    let (mut speed, mut transform, thruster, children, ship) = q_parent.single_mut();
    let movement_direction = transform.rotation * Vec3::Y;

    if thruster.active {
        speed.0 += movement_direction.truncate().normalize() * ship.thrust_accel * TIME_STEP;
        speed.0 = speed.0.clamp_length_max(ship.max_speed);
    }

    // always decelerate a bit
    speed.0 = speed
        .0
        .clamp_length_max(speed.0.length() + ship.idle_accel * TIME_STEP);

    transform.translation += speed.0.extend(0.0) * TIME_STEP;

    for child in children.iter() {
        let mut visibility = q_child.get_mut(*child).unwrap();
        *visibility = if thruster.active {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
}

/// for missile, etc. but *not* for ship
pub fn basic_speed_system(mut query: Query<(&Speed, &mut Transform), Without<Ship>>) {
    for (speed, mut transform) in query.iter_mut() {
        transform.translation += speed.0.extend(0.0) * TIME_STEP;
    }
}

pub fn basic_rotation_speed_system(
    mut query: Query<(&RotationSpeed, &mut Transform), Without<Ship>>,
) {
    for (rot_speed, mut transform) in query.iter_mut() {
        transform.rotate(Quat::from_rotation_z(rot_speed.0 * TIME_STEP))
    }
}

pub fn life_time_system(mut commands: Commands, mut query: Query<(Entity, &mut LifeTime)>) {
    for (entity, mut life_time) in query.iter_mut() {
        life_time.0.tick(Duration::from_secs_f32(TIME_STEP));
        if life_time.0.finished() {
            commands.entity(entity).despawn();
        }
    }
}

pub fn spawn_missiles_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<LineMaterial>>,
    mut commands: Commands,
    q_ship: Query<(&Speed, &Transform), With<Ship>>,
) {
    let (Speed(ship_speed), ship_transform) = q_ship.single();

    let heading_vec = (ship_transform.rotation * Vec3::Y).truncate();
    let heading_speed = ship_speed.dot(heading_vec);

    if keyboard_input.just_pressed(KeyCode::Space) {
        let missile = Missile::default();
        let speed = missile.speed;
        let time_to_live = missile.time_to_live;
        let momentum_transfer = missile.momentum_transfer;

        let mut transform = *ship_transform;
        transform.translation += (heading_vec * Ship::MISSILE_SPAWN_OFFSET).extend(0.0);

        let vertices = [Vec2::new(0.0, 0.0), Vec2::new(0.0, 4.0)];
        commands.spawn((
            missile,
            Speed(heading_vec.normalize() * (heading_speed * momentum_transfer + speed)),
            LifeTime(Timer::from_seconds(time_to_live, TimerMode::Once)),
            LineSpriteBundleBuilder::new(vertices, false)
                .transform(transform)
                .build(&mut meshes, &mut materials),
        ));
    }
}

pub fn keyboard_input_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Thruster, &Ship)>,
) {
    let (mut transform, mut thruster, ship) = query.single_mut();
    if keyboard_input.pressed(KeyCode::Left) {
        transform.rotate(Quat::from_rotation_z(ship.rot_speed * TIME_STEP));
    }
    if keyboard_input.pressed(KeyCode::Right) {
        transform.rotate(Quat::from_rotation_z(-ship.rot_speed * TIME_STEP));
    }

    thruster.active = keyboard_input.pressed(KeyCode::Up);
}

pub fn wrap_positions(resolution: Res<Resolution>, mut query: Query<&mut Transform>) {
    for mut transform in query.iter_mut() {
        if transform.translation.x > resolution.width / 2.0 {
            transform.translation.x = -resolution.width / 2.0;
        } else if transform.translation.x < -resolution.width / 2.0 {
            transform.translation.x = resolution.width / 2.0;
        }
        if transform.translation.y > resolution.height / 2.0 {
            transform.translation.y = -resolution.height / 2.0;
        } else if transform.translation.y < -resolution.height / 2.0 {
            transform.translation.y = resolution.height / 2.0;
        }
    }
}

/// for the initial asteroid spawn
pub fn spawn_asteroids_system(
    mut events: ResMut<Events<AsteroidSpawnEvent>>,
    query: Query<&Asteroid>,
) {
    if query.is_empty() && events.is_empty() {
        events.send(AsteroidSpawnEvent {
            category: 3,
            start_position: None,
            start_speed: None,
        });
    }
}

pub fn asteroid_birth_system(
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<LineMaterial>>,
    mut commands: Commands,
    resolution: Res<Resolution>,
    mut spawn_events: ResMut<Events<AsteroidSpawnEvent>>,
) {
    for e in spawn_events.drain() {
        let rng = &mut rand::thread_rng();
        let size = 10.0 * e.category as f32 + rng.gen_range(-2.0..2.0);

        const NUM_VERTICES: usize = 10;
        let pts = (0..NUM_VERTICES)
            .map(|i| 2.0 * PI * (i as f32 / NUM_VERTICES as f32))
            .map(|a| {
                let rng = &mut rand::thread_rng();
                Vec2::new(
                    a.cos() + rng.gen_range(-0.1..0.1),
                    a.sin() + rng.gen_range(-0.1..0.1),
                )
            });

        let position = e.start_position.unwrap_or_else(|| {
            Vec2::new(
                rng.gen_range(-resolution.width / 2.0..resolution.width / 2.0),
                rng.gen_range(-resolution.height / 2.0..resolution.height / 2.0),
            )
        });

        let speed = e
            .start_speed
            .map(|v| v + Vec2::new(rng.gen_range(-20.0..20.0), rng.gen_range(-20.0..20.0)))
            .unwrap_or_else(|| Vec2::new(rng.gen_range(-50.0..50.0), rng.gen_range(-50.0..50.0)));

        commands.spawn((
            Asteroid {
                category: e.category,
            },
            Speed(speed),
            RotationSpeed(rng.gen_range(-1.0..1.0)),
            LineSpriteBundleBuilder::new(pts, true)
                .transform(
                    Transform::from_translation(position.extend(0.0))
                        .with_scale(Vec3::new(size, size, 1.0)),
                )
                .build(&mut meshes, &mut materials),
        ));
    }
}

pub fn asteroid_kill_system(
    q_asteroid: Query<(Entity, &Transform, &Shape, &Asteroid)>,
    q_missile: Query<(Entity, &Transform, &Shape), With<Missile>>,
    mut commands: Commands,
    mut kill_sender: EventWriter<AsteroidKillEvent>,
    mut spawn_sender: EventWriter<AsteroidSpawnEvent>,
) {
    for (asteroid_entity, asteroid_transform, asteroid_shape, asteroid) in q_asteroid.iter() {
        let asteroid_geom = asteroid_shape.as_geometry(asteroid_transform);

        for (missile_entity, missile_transform, missile_shape) in q_missile.iter() {
            let missile_geom = missile_shape.as_geometry(missile_transform);

            let collision = if let (Some(asteroid_geom), Some(missile_geom)) =
                (&asteroid_geom, &missile_geom)
            {
                asteroid_geom.intersects(missile_geom)
            } else {
                false
            };

            if collision {
                kill_sender.send(AsteroidKillEvent {
                    id: asteroid_entity,
                });
                commands.entity(asteroid_entity).despawn();
                commands.entity(missile_entity).despawn();

                // spawn new asteroids
                if asteroid.category > 1 {
                    let mut rng = rand::thread_rng();
                    for _ in 0..3 {
                        spawn_sender.send(AsteroidSpawnEvent {
                            category: asteroid.category - 1,
                            start_position: Some(asteroid_transform.translation.truncate()),
                            start_speed: Some(Vec2::new(
                                rng.gen_range(-50.0..50.0),
                                rng.gen_range(-50.0..50.0),
                            )),
                        });
                    }
                }
            }
        }
    }
}

pub fn explode_asteroid(mut receiver: EventReader<AsteroidKillEvent>) {
    for _event in receiver.iter() {
        //todo
    }
}
