

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
}

