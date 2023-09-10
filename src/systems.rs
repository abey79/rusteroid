use crate::components::{Flame, LifeTime, Missile, Ship, Speed, Thruster};
use crate::line_sprite::{LineMaterial, LineSprintBundleBuilder};
use crate::{Resolution, TIME_STEP};
use bevy::prelude::*;
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

        let mut transform = ship_transform.clone();
        transform.translation += (heading_vec * Ship::MISSILE_SPAWN_OFFSET).extend(0.0);
        commands.spawn((
            missile,
            Speed(heading_vec.normalize() * (heading_speed * momentum_transfer + speed)),
            LifeTime(Timer::from_seconds(time_to_live, TimerMode::Once)),
            LineSprintBundleBuilder::from_vertices(
                [Vec2::new(0.0, 0.0), Vec2::new(0.0, 4.0)],
                false,
            )
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
