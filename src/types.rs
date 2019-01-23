use std::ops::Mul;

pub type Vector = nalgebra::Vector3<f64>;
pub type PixelColor = image::Rgb<u8>;


#[derive(Copy,Clone)]
pub struct Color{
    pub r: f64,
    pub g: f64,
    pub b: f64
}

impl Color {
    pub fn to_pixel(&self) -> PixelColor{
        PixelColor{data:[(self.r * 255.0) as u8,(self.g * 255.0) as u8,(self.b * 255.0) as u8]}
    }

    pub fn set(&mut self, i:u8){
        self.r = 0.8;self.g = 0.8;self.b = 0.0;
        match i%3 {
            0 => self.b =0.8,
            1 => self.r=0.0,
            2 => self.g = 0.0,
            _ => ()
        }
    }

    pub fn gray(c:f64) ->Color{
        let c = c/4.0;
        Color{r:c,g:c,b:c}
    }
}

impl Mul<f64> for Color{
    type Output = Color;

    fn mul(self, rhs: f64) -> Self {
        Color{r:self.r*rhs,g:self.g*rhs,b:self.b*rhs}
    }
}
// ********************************************************

pub struct Ray{
    pub orig: Vector,
    pub dir: Vector,
}

impl Ray{
    pub fn new(orig: Vector, dir: Vector) ->Ray{

        Ray{orig, dir: dir.normalize()}
    }

    pub fn look_at(orig: Vector, dest: Vector) -> Ray{
        Ray::new(orig, dest-orig)
    }
}

// ********************************************************

pub struct Hit{
    pub t: f64,
    pub n: Vector,
    pub c: Color
}

// ********************************************************
