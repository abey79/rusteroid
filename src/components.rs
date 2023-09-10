use bevy::prelude::*;

#[derive(Component)]
pub struct Ship {
    pub thrust_accel: f32,
    pub idle_accel: f32,
    pub max_speed: f32,
    pub rot_speed: f32,
}

impl Default for Ship {
    fn default() -> Self {
        Self {
            thrust_accel: 300.0,
            idle_accel: -10.0,
            max_speed: 450.0,
            rot_speed: 5.0,
        }
    }
}

#[derive(Component, Debug, Default)]
pub struct Thruster {
    pub active: bool,
}

#[derive(Component)]
pub struct Flame;

#[derive(Component, Debug, Default)]
pub struct Speed(pub Vec2);
