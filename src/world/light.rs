use crate::{Color, Vector};
use rand::Rng;

#[allow(non_snake_case)]
pub struct Light {
    orig: Vector,
    a: Vector,
    b: Vector,
    pub(super) n: Vector,
    pub(super) I: Color,
    pub(super) A: f64,
}

impl Light {
    #[allow(non_snake_case)]
    pub fn new(center: Vector, a: Vector, b: Vector, c: Color) -> Light {
        let cross = a.cross(&b);
        let A = cross.norm();
        Light {
            orig: center - a / 2.0 - b / 2.0,
            a,
            b,
            n: cross.normalize(),
            A,
            I: c / A,
        }
    }

    pub fn default() -> Light {
        Light::new(
            Vector::new(-0.25, 0.0, -0.25),
            Vector::new(0.5, 0.0, 0.0),
            Vector::new(0.0, 0.0, 0.5),
            Color::new(1.0, 1.0, 1.0),
        )
    }

    pub fn get_sample_points<R: Rng>(&self, rng: &mut R) -> [Vector; 4] {
        [
            self.orig + self.a * rng.gen::<f64>() * 0.5 + self.b * rng.gen::<f64>() * 0.5,
            self.orig + self.a * (rng.gen::<f64>() * 0.5 + 0.5) + self.b * rng.gen::<f64>() * 0.5,
            self.orig + self.a * rng.gen::<f64>() * 0.5 + self.b * (rng.gen::<f64>() * 0.5 + 0.5),
            self.orig
                + self.a * (rng.gen::<f64>() * 0.5 + 0.5)
                + self.b * (rng.gen::<f64>() * 0.5 + 0.5),
        ]
    }
}
