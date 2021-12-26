mod basic_container;
mod bvh;
pub(crate) mod container;
pub mod entities;
mod light;

pub use basic_container::BasicCreator;
pub use bvh::SimpleBVHCreator;
pub use container::{Container, ContainerCreator};
pub use light::Light;

use crate::types::*;
use crate::world::entities::Entity;

use crate::world::container::EntitiesAdder;
use rand::rngs::SmallRng;
use rand::{FromEntropy, Rng};
use std::f64;
use std::f64::consts::PI;
use std::sync::Arc;

pub struct WorldBuilder<E> {
    entities: Vec<E>,
    pub light: Light,
}

impl<E> WorldBuilder<E> {
    pub fn from_entities(entities: Vec<E>) -> WorldBuilder<E> {
        WorldBuilder {
            entities,
            light: Light::default(),
        }
    }

    pub fn new() -> WorldBuilder<E> {
        WorldBuilder {
            entities: Vec::new(),
            light: Light::default(),
        }
    }

    pub fn add_light(&mut self, l: Light) {
        self.light = l;
    }

    pub fn add_entity(&mut self, e: E) {
        self.entities.push(e);
    }
}

impl<E: Entity> WorldBuilder<E> {
    pub fn build<CC: EntitiesAdder<E>>(self) -> Arc<World<CC::Output>> {
        let mut cc = CC::new();
        cc.add_entities(self.entities);
        let w = World {
            entities: cc.create(),
            light: self.light,
        };
        Arc::new(w)
    }
}

//****************************************************

pub struct World<C> {
    entities: C,
    light: Light,
}

impl<C: Container> World<C> {
    pub fn shoot_ray(&self, ray: Ray, steps: u32) -> Color {
        let mut rng = SmallRng::from_entropy();
        RayBouncer::new(steps, ray, &self.entities)
            .scan(Color::white(), |c, h| {
                *c *= h.c;
                Some(*c * self.get_radiance_from_light(&h.p, &h.n, &mut rng))
            })
            .reduce(|c1, c2| c1 + c2)
            .unwrap_or_else(Color::black)
    }

    fn get_radiance_from_light<R: Rng>(&self, p: &Vector, n: &Vector, rng: &mut R) -> Color {
        let est: f64 = self
            .light
            .get_sample_points(rng)
            .iter()
            .map(|s| {
                let dir = p - s;
                let ray = Ray::new(*s, dir);
                if self.is_something_blocking(&ray) {
                    0.0
                } else {
                    let dir = dir.normalize();
                    self.light.n.dot(&dir).abs() * -n.dot(&dir) / dir.norm_squared()
                }
            })
            .sum();
        self.light.I * (est * self.light.A / (PI)).max(0.0)
    }

    fn is_something_blocking(&self, ray: &Ray) -> bool {
        self.entities.any_hit(ray, |h| h.t < 1.0)
    }
}

struct RayBouncer<'a, C> {
    ray: Ray,
    entities: &'a C,
    steps: u32,
    rng: SmallRng,
}

impl<'a, C> RayBouncer<'a, C> {
    fn new(steps: u32, ray: Ray, entities: &'a C) -> Self {
        RayBouncer {
            ray,
            entities,
            steps,
            rng: SmallRng::from_entropy(),
        }
    }
}

impl<'a, C: Container> Iterator for RayBouncer<'a, C> {
    type Item = Hit;

    fn next(&mut self) -> Option<Self::Item> {
        if self.steps == 0 {
            None
        } else {
            if let Some(hit) = self.entities.closest_hit(&self.ray) {
                let dir = random_dir_in_hemisphere(&hit.n, &mut self.rng);
                self.ray = Ray::new(hit.p, dir);
                //let R = 2.0 * (hit.n * hit.n.transpose()) - Matrix3::identity();
                //self.ray = Ray::new(hit.p, -R * self.ray.dir);
                self.steps -= 1;
                Some(hit.clone())
            } else {
                None
            }
        }
    }
}

fn random_dir_in_hemisphere<R: Rng>(n: &Vector, rng: &mut R) -> Vector {
    let a = if n.y.powi(2) + n.x.powi(2) > 0.1 {
        Vector::new(n.y, -n.x, 0.0).normalize()
    } else {
        Vector::new(n.z, 0.0, -n.x).normalize()
    };

    let r = rng.gen::<f64>();
    let angle = rng.gen::<f64>() * 2.0 * PI;
    let (sin, cos) = angle.sin_cos();
    let x = cos * r;
    let y = sin * r;
    //    let z = r.hypot(1.0);
    let z = (1.0 - r.powi(2)).sqrt();

    let b = a.cross(n);

    x * a + y * b + z * n
}
//****************************************************
