pub mod entities;

use crate::types::*;

use std::f64;
use self::entities::Entity;


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
        WorldBuilder{entities:Vec::new(),bg_color:Color{r:0.0,g:0.0,b:0.0}}
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
            match  e.as_ref().hit(ray){
                Some(hit) => {
                    if hit.t < t {t=hit.t;c=Some(hit.c);}
                }
                _ => ()
            }

        }
        c.unwrap_or(self.bg_color)

    }
}



