pub mod entities;

use crate::types::*;

use std::f64;
use self::entities::Entity;


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
//****************************************************

pub struct World{
    entities: Vec<Box<Entity>>,
    bg_color: Color
}

impl World{

    pub fn shoot_ray(&self, ray: &Ray) -> Color{
        let mut t = f64::INFINITY;
        let mut h: Option<Hit> =None;
        for e in &self.entities{
            /*match  e.as_ref().hit(ray){
                Some(hit) => {
                    if hit.t < t {
                        t=hit.t; h = Some(hit);
                    }
                }

                _ => ()
            }*/
            if let Some(hit) = e.as_ref().hit(ray){
                if hit.t < t {
                    t=hit.t;
                    h=Some(hit);
                }
            }

        }
        match h {
            Some(h) => h.c * -ray.dir.dot(&h.n),
            _ => self.bg_color
        }


    }
}
//****************************************************

pub struct Light{

}


