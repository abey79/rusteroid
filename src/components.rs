use bevy::prelude::*;

// ============================================
// Common

#[derive(Component, Debug, Default)]
pub struct Speed(pub Vec2);

#[derive(Component, Debug, Default)]
pub struct RotationSpeed(pub f32);

#[derive(Component, Debug, Default)]
pub struct LifeTime(pub Timer);

// ============================================
// Ship

#[derive(Component)]
pub struct Ship {
    pub thrust_accel: f32,
    pub idle_accel: f32,
    pub max_speed: f32,
    pub rot_speed: f32,
}

impl Ship {
    pub const MISSILE_SPAWN_OFFSET: f32 = 10.0;
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

// ============================================
// Missile

#[derive(Component)]
pub struct Missile {
    pub time_to_live: f32,
    pub speed: f32,
    pub momentum_transfer: f32,
}

impl Default for Missile {
    fn default() -> Self {
        Self {
            time_to_live: 1.0,
            speed: 450.0,
            momentum_transfer: 0.2,
        }
    }
}

// ============================================
// Asteroid

#[derive(Component, Debug)]
pub struct Asteroid {
    /// "size" of the asteroid, breaks into asteroids of category `category - 1`
    pub category: u8,
}
