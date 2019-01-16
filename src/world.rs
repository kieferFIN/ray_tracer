use crate::types::*;
use crate::utils::Ray;

use std::f64;
/*
builder!(WorldBuilder => World{
    entitys: Vec<u32> = Vec::new()
});*/

pub struct World{
    entities: Vec<Box<dyn Entity>>,
    bg_color: Color
}

pub struct WorldBuilder {
    pub entities: Vec<Box<dyn Entity>>,
    pub bg_color: Color

}

impl WorldBuilder{
    pub fn new() -> WorldBuilder{
        WorldBuilder{entities:Vec::new(),bg_color:Color{data:[0,0,0]}}
    }

    pub fn build(self) -> World {
        World{entities: self.entities, bg_color: self.bg_color}
    }

}



impl World{

    pub fn shoot_ray(&self, ray: &Ray) -> Color{
        let mut t = f64::INFINITY;
        let mut c: Option<Color> =None;
        for e in &self.entities{
            match e.as_ref().hit(ray){
                Some((t_v, c_v)) => if t_v<t {t=t_v;c=Some(c_v); },
                _ => ()
            };

        }
        c.unwrap_or(self.bg_color)

    }
}

pub trait Entity{
    fn hit(&self, ray: &Ray) -> Option<(f64,Color)>;
}

