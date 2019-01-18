pub mod entities;

use crate::types::*;

use std::f64;
use self::entities::Entity;


pub struct World{
    entities: Vec<Box<Entity>>,
    bg_color: Color
}

pub struct WorldBuilder {
    entities: Vec<Box< Entity>>,
    pub bg_color: Color

}

impl WorldBuilder{
    pub fn from_entities(entities:Vec<Box<Entity>>) -> WorldBuilder{
        WorldBuilder{entities, bg_color: Color::gray(0.0)}
    }

    pub fn new() -> WorldBuilder{
        WorldBuilder{entities:Vec::new(),bg_color:Color::gray(0.0)}
    }

    pub fn add_entity<T: Entity+ 'static >(&mut self, entity:T){
        self.entities.push(Box::new(entity));
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



