//! Kindly contributed by @Wyth@mastodon.art

use crate::asteroids::AsteroidMaker;
use crate::line_sprite::Shape;
use bevy::math::Vec2;
use geo::{BooleanOps, Polygon, Rotate, Scale};
use rand::Rng;
use rand_distr::{Distribution, Normal};
use std::f32::consts::PI;

pub struct PolySpinSmallerAsteroid;

impl AsteroidMaker for PolySpinSmallerAsteroid {
    fn shape_and_segments(&self, category: u8) -> (Shape, Vec<(Vec2, Vec2)>) {
        let rng = &mut rand::thread_rng();

        let times_to_reduce = 2 + category as usize;
        let scale_amount = 0.53 + 0.08 * category as f32;

        let rotation_angle = rng.gen_range(30.0..110.0);

        let base_poly = generate_polygon(1.0, 0.9, 0.13, 18);

        let mut prev_poly = base_poly.clone();
        let mut mask: geo::MultiPolygon<f32> = base_poly.clone().into();

        let mut extra_segments = Vec::new();

        for i in 0..times_to_reduce {
            let new_poly = prev_poly
                .scale(scale_amount)
                .rotate_around_point(rotation_angle * (i + 1) as f32, (0.0, 0.0).into());

            let line_string = new_poly.exterior().clone();

            let mls: geo::MultiLineString<f32> = line_string.into();
            let clipped_lines = mask.clip(&mls, false);

            for line in clipped_lines {
                line.into_points().windows(2).for_each(|pts| {
                    extra_segments.push((point_to_vec(pts[0]), point_to_vec(pts[1])));
                });
            }

            mask = new_poly.intersection(&prev_poly);
            prev_poly = new_poly;
        }

        (
            Shape::from_vertices(base_poly.exterior().points().map(point_to_vec), true),
            extra_segments,
        )
    }
}

fn point_to_vec(p: geo::Point<f32>) -> Vec2 {
    Vec2::new(p.x(), p.y())
}

fn generate_polygon(
    avg_radius: f32,
    mut irregularity: f32,
    mut spikiness: f32,
    num_vertices: usize,
) -> Polygon<f32> {
    irregularity *= 2.0 * PI / num_vertices as f32;
    spikiness *= avg_radius;
    let normal = Normal::new(avg_radius, spikiness).unwrap();

    let mut rng = rand::thread_rng();
    let mut points = Vec::new();
    let mut angle = rng.gen_range(0.0..2.0 * PI);
    for _ in 0..num_vertices {
        let radius = normal.sample(&mut rng).max(0.0).min(2.0 * avg_radius);
        let point = (radius * angle.cos(), radius * angle.sin());
        points.push(point);
        angle += irregularity;
    }

    Polygon::new(geo::LineString::from(points), vec![])
}
