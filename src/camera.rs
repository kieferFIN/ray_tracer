use crate::world::World;
use crate::types::*;

use image::ImageBuffer;
use image::imageops::resize;
use image::FilterType;

use std::thread;
use std::sync::mpsc;
use std::sync::Arc;

use rand::rngs::SmallRng;
use rand::FromEntropy;
use rand::Rng;


pub struct Camera {
    orig: Vector,
    width: u32,
    height: u32,
    upper_left: Vector,
    dy: Vector,
    dx: Vector,
    threads: u32,
    pic_width: u32,
    pic_height: u32,
    steps: u32,
    rays_per_pixel: u32,
}

pub struct CameraBuilder {
    pub   orig: Vector,
    pub   size: (u32, u32),
    pub   dir: Vector,
    pub   up: Vector,
    pub   horizontal_angle: u32,
    pub   super_sampling_factor:u32,
    pub   rays_per_pixel:u32,
    pub   steps:u32,
    pub   threads:u32

}

impl CameraBuilder {
    pub fn new() -> CameraBuilder {
        CameraBuilder {
            orig: Vector::repeat(0.0),
            size: (400, 300),
            dir: -Vector::z(),
            up: Vector::y(),
            horizontal_angle: 65,
            super_sampling_factor: 2,
            rays_per_pixel:5,
            steps: 3,
            threads: 4
        }
    }

    pub fn look_at(&mut self, at:Vector){
        self.dir = at-self.orig;
    }

    pub fn build(&self) -> Camera {
        let orig =self.orig;
        let width= self.size.0*self.super_sampling_factor;
        let height= self.size.1*self.super_sampling_factor;
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
            let upper_left = orig + dir - width_vector / 2.0 - height_vector / 2.0;
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
            dx,
            threads: self.threads,
            pic_width: self.size.0,
            pic_height: self.size.1,
            rays_per_pixel: self.rays_per_pixel,
            steps: self.steps

        }
    }
}


impl Camera{
    pub fn take_pic(&self, world: & Arc<World>) -> ImageBuffer<PixelColor,Vec<u8>> {
        let mut pic = ImageBuffer::new(self.width,self.height);

        let width_per_thread = self.width/self.threads;
        let height_per_thread = self.height/self.threads;
         println!("{} {}",width_per_thread,height_per_thread);

        let mut thread_container = vec![];
        let (sender, receiver) = mpsc::channel();

        for i in 0..self.threads*self.threads{
            let s = mpsc::Sender::clone(&sender);
            let x0 = i%self.threads * width_per_thread;
            let y0 = i/self.threads * height_per_thread;
            let upper_left =  self.upper_left + self.dx * x0 as f64 + self.dy * y0 as f64;
            let dx = self.dx;
            let dy = self.dy;
            let orig = self.orig;
            let w = Arc::clone(world);
            let steps = self.steps;
            let rpp = self.rays_per_pixel;

            let t = thread::spawn( move || {
                let mut rng = SmallRng::from_entropy();
                //println!("{} {} start",x0, y0);
                for y in 0..height_per_thread{
                    for x in 0..width_per_thread{
                        let mut sum=Color::black();
                        let mut weight_sum = 0.0;
                        for _ in 0..rpp{
                            let x_off=rng.gen::<f64>();
                            let y_off = rng.gen::<f64>();
                            let weight = (x_off-0.5).powi(2)+ (y_off-0.5).powi(2);
                            let c = w.shoot_ray(Ray::look_at(orig,upper_left + dx * (x as f64+x_off) + dy * (y as f64+y_off)),steps);
                            sum += c.iter().fold(Color::black(), |a, b| a+*b ) * weight;
                            weight_sum += weight;
                            /*if c.len()>=3{
                                s.send((x+x0, y+y0, c[2])).unwrap();
                            } else { s.send((x+x0, y+y0, Color::black())).unwrap(); }*/
                            //s.send((x+x0, y+y0, c.iter().fold(Color::black(), |a, b| a+*b ))).unwrap();
                            //s.send((x+x0, y+y0, c)).unwrap();
                        }
                        s.send((x+x0, y+y0, sum/weight_sum)).unwrap();

                    }

                }
                //println!("{} {} end",x0, y0 );
            });
            thread_container.push(t);
        };
        drop(sender);

        for (x,y,c) in receiver{
            pic.put_pixel(x,y,c.to_pixel());
        }

        for t in thread_container {
            t.join().unwrap();
        };

        resize(&pic,self.pic_width, self.pic_height, FilterType::Gaussian)
    }
}