mod basic;
mod poly_spin_smaller;
mod poly_vor_diag;
mod utils;

use crate::line_sprite::Shape;
use bevy::prelude::*;
use rand::Rng;
use vsvg::{DocumentTrait, LayerTrait, PathTrait};

pub trait AsteroidMaker: Sync + Send {
    fn poly_and_sketch(&self, category: u8) -> (Vec<vsvg::Point>, vsvg_sketch::Sketch);

    /// Should be scaled for a radius of approx 1.0.
    fn shape_and_segments(&self, category: u8) -> (Shape, Vec<(Vec2, Vec2)>) {
        let (pts, sketch) = self.poly_and_sketch(category);

        // reasonable tolerance for scale ~1 things
        let tolerance = 0.05;

        let segments = sketch
            .document()
            .flatten(tolerance)
            .layers()
            .values()
            .flat_map(|layer| {
                layer.paths().iter().flat_map(|path| {
                    path.data().points().windows(2).map(|pts| {
                        (
                            Vec2::new(pts[0].x() as f32, pts[0].y() as f32),
                            Vec2::new(pts[1].x() as f32, pts[1].y() as f32),
                        )
                    })
                })
            })
            .collect();

        (
            Shape::from_vertices(
                pts.into_iter()
                    .map(|pt| Vec2::new(pt.x() as f32, pt.y() as f32)),
                true,
            ),
            segments,
        )
    }
}

#[derive(Resource)]
pub struct AsteroidMakerRegistry {
    makers: Vec<Box<dyn AsteroidMaker>>,
}

impl AsteroidMakerRegistry {
    pub fn get_random(&self) -> &dyn AsteroidMaker {
        let rng = &mut rand::thread_rng();
        let index = rng.gen_range(0..self.makers.len());
        self.makers[index].as_ref()
    }
}

pub struct AsteroidMakerPlugin;

impl Plugin for AsteroidMakerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AsteroidMakerRegistry {
            makers: vec![
                Box::new(basic::BasicAsteroid),
                Box::new(poly_vor_diag::PolyVorDiagAsteroid),
                Box::new(poly_spin_smaller::PolySpinSmallerAsteroid),
            ],
        });
    }
}
