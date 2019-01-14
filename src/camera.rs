

use nalgebra::Vector3;
use image::{ImageBuffer, Rgb};
use crate::world::World;

builder!(CamBuilder => Camera{
    orig: Vector3<f64> = Vector3::repeat(0.0),
    size: (u32,u32) = (600,400),
    dir: Vector3<f64> = Vector3::new(0.0,0.0,1.0),
    up: Vector3<f64>= Vector3::new(0.0,1.0,0.0)
});

impl Camera{
    pub fn take_pic(&self, world: &World) -> ImageBuffer<Rgb<u8>,Vec<u8>> {
        ImageBuffer::new(self.size.0,self.size.1)

    }
}
/*
pub struct Camera{
    orig: Vector3<f64>,
    size: (u32,u32),
    dir: Vector3<f64>,
    up: Vector3<f64>

}

impl Camera{
    pub fn take_pic(&self, world: &World) -> ImageBuffer<Rgb<u8>,Vec<u8>> {
        ImageBuffer::new(self.size.0,self.size.1)

    }
}

pub struct Config{
    pub orig: Vector3<f64>,
    pub size: (u32,u32),
    pub dir: Vector3<f64>,
    pub up: Vector3<f64>

}

impl Config{
    pub fn build(&self)->Camera{
        Camera{orig:self.orig,size:self.size,dir:self.dir,up:self.up}

    }

    pub fn new()->Config{
        Config{orig:Vector3::repeat(0.0),size:(600,400),dir:Vector3::new(0.0,0.0,1.0),up:Vector3::new(0.0,1.0,0.0)}
    }

    pub fn s(&mut self, x:(u32,u32)) -> &mut Self{
        self.size = x;
        self
    }
}*/