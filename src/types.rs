use std::ops::{Add, AddAssign, Div, Mul, MulAssign};

pub type Vector = nalgebra::Vector3<f64>;
pub type PixelColor = image::Rgb<u8>;

pub trait ToVector {
    fn to_vector(self) -> Vector;
}

impl ToVector for (f64, f64, f64) {
    fn to_vector(self) -> Vector {
        Vector::new(self.0, self.1, self.2)
    }
}

#[derive(Copy, Clone)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Color {
        Color { r, g, b }
    }

    pub fn black() -> Color {
        Color::gray(0.0)
    }
    pub fn white() -> Color {
        Color::gray(1.0)
    }

    pub fn gray(c: f64) -> Color {
        Color { r: c, g: c, b: c }
    }
}

impl From<(f64, f64, f64)> for Color {
    fn from(t: (f64, f64, f64)) -> Self {
        Color::new(t.0, t.1, t.2)
    }
}

impl From<&Color> for PixelColor {
    fn from(c: &Color) -> Self {
        image::Rgb([
            (c.r.max(0.0).min(1.0) * 255.0) as u8,
            (c.g.max(0.0).min(1.0) * 255.0) as u8,
            (c.b.max(0.0).min(1.0) * 255.0) as u8,
        ])
    }
}

impl Mul<f64> for Color {
    type Output = Color;

    fn mul(self, rhs: f64) -> Self {
        Color {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
        }
    }
}

impl Div<f64> for Color {
    type Output = Color;

    fn div(self, rhs: f64) -> Self::Output {
        Color {
            r: self.r / rhs,
            g: self.g / rhs,
            b: self.b / rhs,
        }
    }
}

impl Mul<Color> for Color {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        Color {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
        }
    }
}

impl Add<Color> for Color {
    type Output = Color;
    fn add(self, rhs: Color) -> Self::Output {
        Color {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}

impl MulAssign<Color> for Color {
    fn mul_assign(&mut self, rhs: Color) {
        self.r *= rhs.r;
        self.g *= rhs.g;
        self.b *= rhs.b;
    }
}

impl AddAssign<Color> for Color {
    fn add_assign(&mut self, rhs: Color) {
        self.r += rhs.r;
        self.g += rhs.g;
        self.b += rhs.b;
    }
}
// ********************************************************

pub struct Ray {
    pub orig: Vector,
    pub dir: Vector,
}

impl Ray {
    pub fn new(orig: Vector, dir: Vector) -> Ray {
        Ray { orig, dir }
    }

    pub fn look_at(orig: Vector, dest: Vector) -> Ray {
        Ray::new(orig, dest - orig)
    }
}

// ********************************************************

pub struct Hit {
    pub t: f64,
    pub n: Vector,
    pub c: Color,
    pub p: Vector,
}

// ********************************************************
