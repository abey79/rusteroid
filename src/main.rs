mod components;
mod line_sprite;
mod systems;

use crate::components::{Flame, Ship, Speed, Thruster};
use crate::line_sprite::{LineMaterial, LineSprintBundleBuilder, LineSpritePlugin};
use crate::systems::{
    basic_speed_system, keyboard_input_system, life_time_system, ship_motion_system,
    spawn_missiles_system, wrap_positions,
};
use bevy::prelude::*;
use bevy::window::WindowResized;

const TIME_STEP: f32 = 1.0 / 60.0;

const INITIAL_WIDTH: f32 = 800.0;
const INITIAL_HEIGHT: f32 = 600.0;

#[derive(Resource)]
struct FrameTimer(Timer);

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<LineMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    // Spawn a list of lines with start and end points for each lines
    let parent = commands
        .spawn((
            Ship::default(),
            Speed::default(),
            Thruster::default(),
            LineSprintBundleBuilder::from_vertices(
                [
                    Vec2::new(-10.0, -5.0),
                    Vec2::new(10.0, -5.0),
                    Vec2::new(0.0, 15.0),
                ],
                true,
            )
            .build(&mut meshes, &mut materials),
        ))
        .id();

    let child = commands
        .spawn((
            Flame,
            LineSprintBundleBuilder::from_vertices(
                [
                    Vec2::new(-5.0, -8.0),
                    Vec2::new(5.0, -8.0),
                    Vec2::new(0.0, -13.0),
                ],
                true,
            )
            .build(&mut meshes, &mut materials),
        ))
        .id();

    commands.entity(parent).push_children(&[child]);
}

trait ResExt {
    fn as_vec3(&self) -> Vec3;
}

#[derive(Resource, Debug, Default)]
pub struct Resolution {
    pub width: f32,
    pub height: f32,
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
        .add_systems(First, (spawn_missiles_system,)) // dont miss key-presses
        .add_systems(
            FixedUpdate,
            (
                keyboard_input_system.before(ship_motion_system),
                ship_motion_system,
                life_time_system,
                basic_speed_system,
                on_resize_system,
                wrap_positions,
            ),
        )
        .run();
}
