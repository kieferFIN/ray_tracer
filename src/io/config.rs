use crate::camera::Camera;
use crate::world::container::{ContainerCreator, EntitiesAdder};
use crate::world::entities::Triangle;
use crate::world::{Light, World};
use crate::{read_obj_file, CameraBuilder, ToVector, WorldBuilder};
use serde::Deserialize;
use std::sync::Arc;

#[derive(Debug, Deserialize)]
pub struct Config {
    world: Option<W>,
    camera: Option<C>,
}

impl Config {
    pub fn create_world<CC: EntitiesAdder<Triangle>>(&self) -> Arc<World<CC::Output>> {
        match &self.world {
            Some(w) => w.create::<CC>(),
            _ => {
                println!("No world specified, default created");
                WorldBuilder::new().build::<CC>()
            }
        }
    }

    pub fn create_camera(&self) -> Camera {
        match &self.camera {
            Some(c) => c.create(),
            _ => {
                println!("No camera specified, default created");
                CameraBuilder::new().build()
            }
        }
    }
}

#[derive(Debug, Deserialize)]
struct W {
    file: Option<String>,
    light: Option<L>,
}

impl W {
    fn create<CC: EntitiesAdder<Triangle>>(&self) -> Arc<World<CC::Output>> {
        let mut wb = match &self.file {
            Some(file) => WorldBuilder::from_entities(read_obj_file(&file).unwrap()),
            _ => WorldBuilder::new(),
        };
        if let Some(l) = &self.light {
            wb.add_light(l.create());
        } else {
            println!("No light specified, using default");
        };
        wb.build::<CC>()
    }
}

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
struct L {
    center: (f64, f64, f64),
    a: (f64, f64, f64),
    b: (f64, f64, f64),
    I: (f64, f64, f64),
}

impl L {
    fn create(&self) -> Light {
        Light::new(
            self.center.to_vector(),
            self.a.to_vector(),
            self.b.to_vector(),
            self.I.into(),
        )
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
        if let Some(v) = self.orig {
            cb.orig = v.to_vector();
        }
        if let Some(v) = self.size {
            cb.size = v;
        }
        if let Some(v) = self.look_at {
            cb.look_at(v.to_vector());
        }
        if let Some(v) = self.up {
            cb.up = v.to_vector();
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
