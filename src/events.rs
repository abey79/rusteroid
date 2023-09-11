use bevy::prelude::*;

#[derive(Event)]
pub struct AsteroidKillEvent {
    pub id: Entity,
}

#[derive(Event)]
pub struct AsteroidSpawnEvent {
    pub category: u8,
    pub start_position: Option<Vec2>,
    pub start_speed: Option<Vec2>,
}
