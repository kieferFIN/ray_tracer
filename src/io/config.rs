use crate::camera::Camera;
use crate::world::entities::Triangle;
use crate::world::World;
use crate::{read_obj_file, CameraBuilder, Vector, WorldBuilder};
use serde::Deserialize;
use std::sync::Arc;

#[derive(Debug, Deserialize)]
pub struct Config {
    world: Option<W>,
    camera: Option<C>,
}

impl Config {
    pub fn create_world(&self) -> Arc<World<Triangle>> {
        match &self.world {
            Some(w) => w.create(),
            _ => WorldBuilder::new().build(),
        }
    }

    pub fn create_camera(&self) -> Camera {
        match &self.camera {
            Some(c) => c.create(),
            _ => CameraBuilder::new().build(),
        }
    }
}

#[derive(Debug, Deserialize)]
struct W {
    file: Option<String>,
}

impl W {
    fn create(&self) -> Arc<World<Triangle>> {
        match &self.file {
            Some(file) => WorldBuilder::from_entities(read_obj_file(&file).unwrap()).build(),
            _ => WorldBuilder::new().build(),
        }
    }
}

#[derive(Debug, Deserialize)]
struct C {
    pub orig: Option<(f64, f64, f64)>,
    pub size: Option<(u32, u32)>,
    pub look_at: Option<(f64, f64, f64)>,
    pub up: Option<(f64, f64, f64)>,
    pub horizontal_angle: Option<u32>,
    pub super_sampling_factor: Option<u32>,
    pub rays_per_pixel: Option<u32>,
    pub steps: Option<u32>,
    pub threads: Option<u32>,
}

impl C {
    fn create(&self) -> Camera {
        let mut cb = CameraBuilder::new();
        if let Some((x, y, z)) = self.orig {
            cb.orig = Vector::new(x, y, z);
        }
        if let Some(v) = self.size {
            cb.size = v;
        }
        if let Some((x, y, z)) = self.look_at {
            cb.look_at(Vector::new(x, y, z));
        }
        if let Some((x, y, z)) = self.up {
            cb.up = Vector::new(x, y, z);
        }
        if let Some(v) = self.super_sampling_factor {
            cb.super_sampling_factor = v;
        }
        if let Some(v) = self.rays_per_pixel {
            cb.rays_per_pixel = v;
        }
        if let Some(v) = self.steps {
            cb.steps = v;
        }
        if let Some(v) = self.threads {
            cb.threads = v;
        }
        cb.build()
    }
}
