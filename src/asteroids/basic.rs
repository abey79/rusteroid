use crate::asteroids::AsteroidMaker;
use rand::Rng;
use std::f64::consts::PI;

pub struct BasicAsteroid;

impl AsteroidMaker for BasicAsteroid {
    fn poly_and_sketch(&self, _category: u8) -> (Vec<vsvg::Point>, vsvg_sketch::Sketch) {
        let rng = &mut rand::thread_rng();

        const NUM_VERTICES: usize = 10;
        let pts = (0..NUM_VERTICES)
            .map(|i| 2.0 * PI * (i as f64 / NUM_VERTICES as f64))
            .map(|a| {
                //let rng = &mut rand::thread_rng();
                vsvg::Point::new(
                    a.cos() + rng.gen_range(-0.1..0.1),
                    a.sin() + rng.gen_range(-0.1..0.1),
                )
            })
            .collect::<Vec<_>>();

        (pts, vsvg_sketch::Sketch::new())
    }
}
