use crate::types::*;
use crate::world::{Container, World};

use image::imageops::{resize, FilterType};
use image::ImageBuffer;

use rand::distributions::Standard;
use std::sync::mpsc;
use std::sync::Arc;

use threadpool::ThreadPool;

use rand::rngs::SmallRng;
use rand::{thread_rng, Rng, SeedableRng};

pub struct Camera {
    orig: Vector,
    width: usize,
    height: usize,
    upper_left: Vector,
    dy: Vector,
    dx: Vector,
    threads: u8,
    block_size: usize,
    pic_width: u32,
    pic_height: u32,
    steps: u8,
    rays_per_pixel: u8,
}

pub struct CameraBuilder {
    pub orig: Vector,
    pub size: (usize, usize),
    pub dir: Vector,
    pub up: Vector,
    pub horizontal_angle: u8,
    pub super_sampling_factor: u8,
    pub rays_per_pixel: u8,
    pub steps: u8,
    pub threads: u8,
    pub block_size: usize,
}

impl CameraBuilder {
    pub fn new() -> CameraBuilder {
        let width = 400;
        CameraBuilder {
            orig: Vector::repeat(0.0),
            size: (width, 300),
            dir: Vector::z(),
            up: Vector::y(),
            horizontal_angle: 65,
            super_sampling_factor: 2,
            rays_per_pixel: 5,
            steps: 3,
            threads: 4,
            block_size: width * 20,
        }
    }

    pub fn look_at(&mut self, at: Vector) {
        self.dir = at - self.orig;
    }

    pub fn build(&self) -> Camera {
        let orig = self.orig;
        let width = self.size.0 * self.super_sampling_factor as usize;
        let height = self.size.1 * self.super_sampling_factor as usize;
        let (upper_left, dx, dy) = {
            let ratio = height as f64 / width as f64;
            let half_angle = (self.horizontal_angle as f64 * 0.5).to_radians();
            let dir = self.dir.normalize();
            let right = dir.cross(&self.up).normalize();
            let down = dir.cross(&right).normalize();

            let w = half_angle.tan() * 2.0;
            let h = w * ratio;
            let width_vector = w * right;
            let height_vector = h * down;
            let dx = width_vector / width as f64;
            let dy = height_vector / height as f64;
            let upper_left = orig + dir - width_vector / 2.0 - height_vector / 2.0;

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
            block_size: self.block_size * self.super_sampling_factor as usize,
            pic_width: self.size.0 as u32,
            pic_height: self.size.1 as u32,
            rays_per_pixel: self.rays_per_pixel,
            steps: self.steps,
        }
    }
}

impl Camera {
    pub fn take_pic<C: Container<Ray, Hit>>(
        &self,
        world: &Arc<World<C>>,
    ) -> ImageBuffer<PixelColor, Vec<u8>> {
        let width = self.width;
        let from_index_to_coordinate = move |i: usize| -> (usize, usize) { (i % width, i / width) };

        let num_pixels = self.width * self.height;
        let mut pic_buffer = vec![0; num_pixels * 3];
        let mut thread_rng = thread_rng();

        let thread_pool = ThreadPool::new(self.threads as usize);
        let num_tasks = num_pixels / self.block_size;
        println!("{}", num_tasks);
        let (sender, receiver) = mpsc::channel();

        for i in 0..num_tasks {
            let s = mpsc::Sender::clone(&sender);
            let block_size = self.block_size;
            let start_index = i * block_size;
            let dx = self.dx;
            let dy = self.dy;
            let upper_left = self.upper_left;
            let orig = self.orig;
            let w = Arc::clone(world);
            let steps = self.steps;
            let rpp = self.rays_per_pixel as usize;
            let mut rng = SmallRng::from_rng(&mut thread_rng).unwrap();

            thread_pool.execute(move || {
                let mut buffer = vec![Color::black(); block_size];
                for p in 0..block_size {
                    let (x, y) = from_index_to_coordinate(p + start_index);
                    //println!("p{} {}:{}", i, y, x);
                    let (sum, weight_sum) = (&mut rng)
                        .sample_iter(&Standard)
                        .take(rpp)
                        .map(|(x_off, y_off): (f64, f64)| {
                            let weight = (x_off - 0.5).powi(2) + (y_off - 0.5).powi(2);
                            let c = w.shoot_ray(
                                Ray::look_at(
                                    orig,
                                    upper_left + dx * (x as f64 + x_off) + dy * (y as f64 + y_off),
                                ),
                                steps,
                            );
                            (c * weight, weight)
                        })
                        .reduce(|(c1, w1), (c2, w2)| (c1 + c2, w1 + w2))
                        .unwrap_or((Color::black(), 1.0));
                    buffer[p] = sum / weight_sum;
                }
                s.send((
                    start_index,
                    buffer
                        .iter()
                        .map(|c| c.to_raw())
                        .flatten()
                        .collect::<Vec<u8>>(),
                ))
                .unwrap();
            });
        }
        drop(sender);
        for (starting_index, buffer) in receiver {
            pic_buffer[starting_index * 3..starting_index * 3 + buffer.len()]
                .copy_from_slice(&buffer);
        }
        let pic = ImageBuffer::from_raw(self.width as u32, self.height as u32, pic_buffer).unwrap();
        eprint!("  Done \n");
        thread_pool.join();
        resize(&pic, self.pic_width, self.pic_height, FilterType::Gaussian)
    }
}
