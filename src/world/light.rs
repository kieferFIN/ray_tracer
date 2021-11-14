use crate::{Color, Vector};
use rand::rngs::SmallRng;
use rand::{FromEntropy, Rng};

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
    pub fn new(orig: Vector, a: Vector, b: Vector, I: Color) -> Light {
        let cross = a.cross(&b);
        Light {
            orig,
            a,
            b,
            n: cross.normalize(),
            A: cross.norm(),
            I,
        }
    }

    pub fn default() -> Light {
        Light::new(
            Vector::new(-0.2, 0.9, -0.2),
            Vector::new(0.4, 0.0, 0.0),
            Vector::new(0.0, 0.0, 0.4),
            Color::new(3.0, 2.9, 2.0),
        )
    }

    pub fn get_sample_points(&self) -> [Vector; 4] {
        let mut rng = SmallRng::from_entropy();
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
