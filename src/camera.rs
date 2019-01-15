
use crate::types::*;

use image::ImageBuffer;
use crate::world::World;
use crate::utils::Ray;


builder!(CamBuilder => Camera{
    orig: Vector= Vector::repeat(0.0),
    size: (u32,u32) = (600,400),
    dir: Vector = Vector::z(),
    up: Vector= Vector::y(),
    horizontal_angle: u32 = 65
});

impl Camera{
    pub fn take_pic(&self, world: &World) -> ImageBuffer<Color,Vec<u8>> {

        let width= self.size.0;
        let height= self.size.1;
        let (upper_left, dx, dy) = {
            let ratio = height as f64 / width as f64;
            let half_angle = (self.horizontal_angle as f64 *0.5).to_radians();
            let orig = self.orig;
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


        let mut pic = ImageBuffer::new(width,height);

        for (x,y,p) in pic.enumerate_pixels_mut(){

            let c = world.shoot_ray(&Ray::look_at(self.orig,upper_left + dx * x as f64 + dy * y as f64));
            *p=c;
        };
        pic
    }
}