pub mod entities;

use crate::types::*;
use self::entities::Triangle;

use std::f64;
use std::f64::consts::PI;
use rand::rngs::SmallRng;
use rand::FromEntropy;
use rand::Rng;
use std::sync::Arc;

pub struct WorldBuilder {
    entities: Vec<Triangle>,
    pub light: Light,
}

impl WorldBuilder {
    pub fn from_triangles(entities: Vec<Triangle>) -> WorldBuilder {
        WorldBuilder { entities, light: Light::default() }
    }

    pub fn new() -> WorldBuilder {
        WorldBuilder { entities: Vec::new(), light: Light::default() }
    }

    pub fn add_triangle(&mut self, t: Triangle) {
        self.entities.push(t);
    }

    pub fn build(self) -> Arc<World> {
        let w = World { triangles: self.entities, light: self.light };
        Arc::new(w)
    }
}
//****************************************************

pub struct World {
    triangles: Vec<Triangle>,
    light: Light,
}

impl World {
    pub fn shoot_ray(&self, mut ray: Ray, steps: u32) -> Vec<Color> {
        let epsilon = 0.01;
        let mut colors = Vec::new();
        let mut c = Color::gray(1.0);

        for _ in 0..steps {
            if let Some(hit) = self.triangles.iter()
                .filter_map(|t| t.hit(&ray))
                .min_by(|h1, h2| h1.t.partial_cmp(&h2.t).unwrap()) {
                c *= hit.c;
                let point = ray.orig + ray.dir * (hit.t - epsilon);
                colors.push(c * self.get_radiance_from_light(&point, &hit.n));
                let dir = random_dir_in_hemisphere(&hit.n);
                ray = Ray::new(point, dir);
            } else {
                break;
            }
        }
        colors
    }


    fn get_radiance_from_light(&self, p: &Vector, n: &Vector) -> Color {
        let samples_from_light = self.light.get_sample_points();
        let mut est: f64 = 0.0;
        for s in samples_from_light.iter() {
            let dir = p - s;
            let ray = Ray::new(*s, dir);
            let dir = dir.normalize();
            if self.is_not_something_blocking(&ray) {
                est += self.light.n.dot(&dir).abs() * -n.dot(&dir) / dir.norm_squared();
            }
        }
        self.light.I * (est * self.light.A / (PI)).max(0.0)
    }


    fn is_not_something_blocking(&self, ray: &Ray) -> bool {
        for e in &self.triangles {
            if let Some(hit) = e.hit(&ray) {
                if hit.t < 1.0 {
                    return false;
                }
            }
        }
        true
    }
}

fn random_dir_in_hemisphere(n: &Vector) -> Vector {
    let a = if n.y.powi(2) + n.x.powi(2) > 0.1 {
        Vector::new(n.y, -n.x, 0.0).normalize()
    } else {
        Vector::new(n.z, 0.0, -n.x).normalize()
    };

    let mut rng = SmallRng::from_entropy();
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

pub struct Light {
    orig: Vector,
    a: Vector,
    b: Vector,
    n: Vector,
    I: Color,
    A: f64,
}

impl Light {
    pub fn new(orig: Vector, a: Vector, b: Vector, I: Color) -> Light {
        let cross = a.cross(&b);
        Light { orig, a, b, n: cross.normalize(), A: cross.norm(), I }
    }

    pub fn default() -> Light {
        Light::new(Vector::new(-0.2, 0.9, -0.2), Vector::new(0.4, 0.0, 0.0), Vector::new(0.0, 0.0, 0.4), Color::new(3.0, 2.9, 2.0))
    }

    pub fn get_sample_points(&self) -> [Vector; 4] {
        let mut rng = SmallRng::from_entropy();
        [
            self.orig + self.a * rng.gen::<f64>() * 0.5 + self.b * rng.gen::<f64>() * 0.5,
            self.orig + self.a * (rng.gen::<f64>() * 0.5 + 0.5) + self.b * rng.gen::<f64>() * 0.5,
            self.orig + self.a * rng.gen::<f64>() * 0.5 + self.b * (rng.gen::<f64>() * 0.5 + 0.5),
            self.orig + self.a * (rng.gen::<f64>() * 0.5 + 0.5) + self.b * (rng.gen::<f64>() * 0.5 + 0.5),
        ]
        //[self.orig , self.orig+self.a, self.orig+self.b, self.orig+self.a+self.b]
    }
}


