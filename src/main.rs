mod line_sprite;

use crate::line_sprite::{LineMaterial, LineSprintBundleBuilder, LineSpritePlugin};
use bevy::prelude::*;
use bevy::window::WindowResized;

const TIME_STEP: f32 = 1.0 / 60.0;

const INITIAL_WIDTH: f32 = 800.0;
const INITIAL_HEIGHT: f32 = 600.0;

#[derive(Component)]
struct Ship {
    thrust_accel: f32,
    idle_accel: f32,
    max_speed: f32,
    rot_speed: f32,
}

#[derive(Component, Debug, Default)]
struct Thruster {
    active: bool,
}

#[derive(Component)]
struct Flame;

#[derive(Component, Debug, Default)]
struct Speed(Vec2);

#[derive(Resource)]
struct FrameTimer(Timer);

fn ship_motion_system(
    mut q_parent: Query<(&mut Speed, &mut Transform, &Thruster, &Children, &Ship)>,
    mut q_child: Query<&mut Visibility, With<Flame>>,
) {
    let (mut speed, mut transform, thruster, children, ship) = q_parent.single_mut();
    let movement_direction = transform.rotation * Vec3::Y;

    if thruster.active {
        speed.0 += movement_direction.truncate().normalize() * ship.thrust_accel * TIME_STEP;
        speed.0 = speed.0.clamp_length_max(ship.max_speed);

        println!("speed: {:?}", speed.0);
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

fn keyboard_input_system(
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

fn wrap_positions(resolution: Res<Resolution>, mut query: Query<&mut Transform>) {
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

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<LineMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    // Spawn a list of lines with start and end points for each lines
    let parent = commands
        .spawn((
            Ship {
                thrust_accel: 300.0,
                idle_accel: -10.0,
                max_speed: 450.0,
                rot_speed: 5.0,
            },
            Speed::default(),
            Thruster::default(),
            LineSprintBundleBuilder::from_vertices([
                Vec2::new(-10.0, -5.0),
                Vec2::new(10.0, -5.0),
                Vec2::new(0.0, 15.0),
            ])
            .build(&mut meshes, &mut materials),
        ))
        .id();

    let child = commands
        .spawn((
            Flame,
            LineSprintBundleBuilder::from_vertices([
                Vec2::new(-5.0, -8.0),
                Vec2::new(5.0, -8.0),
                Vec2::new(0.0, -13.0),
            ])
            .build(&mut meshes, &mut materials),
        ))
        .id();

    commands.entity(parent).push_children(&[child]);
}

trait ResExt {
    fn as_vec3(&self) -> Vec3;
}

#[derive(Resource, Debug, Default)]
struct Resolution {
    width: f32,
    height: f32,
}

impl ResExt for Resolution {
    fn as_vec3(&self) -> Vec3 {
        Vec3::new(self.width, self.height, 0.0)
    }
}

impl ResExt for WindowResized {
    fn as_vec3(&self) -> Vec3 {
        Vec3::new(self.width, self.height, 0.0)
    }
}

fn on_resize_system(
    mut resolution: ResMut<Resolution>,
    mut resize_reader: EventReader<WindowResized>,
    mut query: Query<&mut Transform, With<Ship>>,
) {
    for e in resize_reader.iter() {
        // Adjust position for the new resolution
        for mut transform in query.iter_mut() {
            transform.translation.x += 0.5 * (resolution.width - e.width);
            transform.translation.y -= 0.5 * (resolution.height - e.height);
        }

        // When resolution is being changed
        resolution.width = e.width;
        resolution.height = e.height;
    }
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Rusteroïds".into(),
                    resolution: (INITIAL_WIDTH, INITIAL_HEIGHT).into(),
                    ..default()
                }),
                ..default()
            }),
            LineSpritePlugin,
        ))
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(FixedTime::new_from_secs(TIME_STEP))
        .insert_resource(Resolution {
            width: INITIAL_WIDTH,
            height: INITIAL_HEIGHT,
        })
        .insert_resource(FrameTimer(Timer::from_seconds(1.0, TimerMode::Repeating)))
        .add_systems(Startup, setup)
        .add_systems(
            FixedUpdate,
            (
                ship_motion_system.after(keyboard_input_system),
                on_resize_system,
                wrap_positions,
                keyboard_input_system,
            ),
        )
        .run();
}
