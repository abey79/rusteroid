use bevy::prelude::*;

pub fn geo_point_to_vec(p: geo::Point<f32>) -> Vec2 {
    Vec2::new(p.x(), p.y())
}
