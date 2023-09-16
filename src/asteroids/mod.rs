mod basic;

use crate::line_sprite::Shape;
use bevy::prelude::*;
use rand::Rng;

pub trait AsteroidMaker: Sync + Send {
    fn shape(&self, category: u8) -> Shape;

    fn extra_segments(&self, _category: u8) -> Vec<(Vec2, Vec2)> {
        vec![]
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
            makers: vec![Box::new(basic::BasicAsteroid)],
        });
    }
}
