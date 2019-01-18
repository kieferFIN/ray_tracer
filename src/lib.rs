
mod types;
mod camera;
mod world;
mod io;

pub use self::types::*;
pub use self::world::WorldBuilder;
pub use self::camera::CameraBuilder;
pub use self::io::read_obj_file;
pub use self::world::entities::TestBall;

use std::time::Instant;

pub struct Timer {
    moments: Vec<Instant>
}

impl Timer{
    pub fn new() -> Timer{
        Timer{moments: Vec::new()}
    }


    pub fn add(&mut self){

        self.moments.push(Instant::now());
    }
    pub fn print(&self){
        for i in 1 .. self.moments.len() {
            let dur = self.moments[i].duration_since(self.moments[i-1]);
            print!("{}:{} ", dur.as_secs(),dur.subsec_millis());
        }
    }
}





