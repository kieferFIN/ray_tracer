
use crate::types::*;

use image::ImageBuffer;
use crate::world::World;
use crate::utils::Ray;

/*
builder!(CamBuilder => Camera{
    orig: Vector= Vector::repeat(0.0),
    size: (u32,u32) = (600,400),
    dir: Vector = Vector::z(),
    up: Vector= Vector::y(),
    horizontal_angle: u32 = 65
});
*/
pub struct Camera {
    orig: Vector,
    width: u32,
    height: u32,
    upper_left: Vector,
    dy: Vector,
    dx: Vector
}

pub struct CameraBuilder {
    pub   orig: Vector,
    pub   size: (u32, u32),
    pub   dir: Vector,
    pub   up: Vector,
    pub   horizontal_angle: u32,
}

impl CameraBuilder {
    pub fn new() -> CameraBuilder {
        CameraBuilder {
            orig: Vector::repeat(0.0),
            size: (600, 400),
            dir: Vector::z(),
            up: Vector::y(),
            horizontal_angle: 65,
        }
    }

    pub fn build(&self) -> Camera {
        let orig =self.orig;
        let width= self.size.0;
        let height= self.size.1;
        let (upper_left, dx, dy) = {
            let ratio = height as f64 / width as f64;
            let half_angle = (self.horizontal_angle as f64 *0.5).to_radians();
            let dir = self.dir.normalize();
            let right = dir.cross(&self.up).normalize();
            let down = dir.cross(&right).normalize();

            let w  = half_angle.tan()* 2.0;
            let h = w * ratio;
            let width_vector = w * right;
            let height_vector = h * down;
            let upper_left = orig + dir - down / 2.0 - right / 2.0;
            let dx = width_vector / width as f64;
            let dy = height_vector / height as f64;
            (upper_left, dx, dy)
        };

        Camera {
            orig,
            width,
            height,
            upper_left,
            dy,
            dx

        }
    }
}


impl Camera{
    pub fn take_pic(&self, world: &World) -> ImageBuffer<Color,Vec<u8>> {

        let mut pic = ImageBuffer::new(self.width,self.height);

        for (x,y,p) in pic.enumerate_pixels_mut(){

            let c = world.shoot_ray(&Ray::look_at(self.orig,self.upper_left + self.dx * x as f64 + self.dy * y as f64));
            *p=c;
        };
        pic
    }
}