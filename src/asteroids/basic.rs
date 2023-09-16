use crate::asteroids::AsteroidMaker;
use crate::line_sprite::Shape;
use bevy::prelude::*;
use rand::Rng;
use std::f32::consts::PI;

pub struct BasicAsteroid;

impl AsteroidMaker for BasicAsteroid {
    fn shape_and_segments(&self, _category: u8) -> (Shape, Vec<(Vec2, Vec2)>) {
        let rng = &mut rand::thread_rng();

        const NUM_VERTICES: usize = 10;
        let pts = (0..NUM_VERTICES)
            .map(|i| 2.0 * PI * (i as f32 / NUM_VERTICES as f32))
            .map(|a| {
                //let rng = &mut rand::thread_rng();
                Vec2::new(
                    a.cos() + rng.gen_range(-0.1..0.1),
                    a.sin() + rng.gen_range(-0.1..0.1),
                )
            });

        (Shape::from_vertices(pts, true), vec![])
    }
}
