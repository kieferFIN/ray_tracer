pub mod entities;

use crate::types::*;
use self::entities::Entity;

use std::f64;
use std::f64::consts::PI;
use rand::rngs::SmallRng;
use rand::FromEntropy;
use rand::Rng;

pub struct WorldBuilder {
    entities: Vec<Box< Entity>>,
    pub bg_color: Color,
    pub light: Light


}

impl WorldBuilder{
    pub fn from_entities(entities:Vec<Box<Entity>>) -> WorldBuilder{
        WorldBuilder{entities, bg_color: Color::gray(0.0), light: Light::default()}
    }

    pub fn new() -> WorldBuilder{
        WorldBuilder{entities:Vec::new(),bg_color:Color::gray(0.0), light:Light::default()}
    }

    pub fn add_entity<T: Entity+ 'static >(&mut self, entity:T){
        self.entities.push(Box::new(entity));
    }

    pub fn build(self) -> World {
        World{entities: self.entities, bg_color: self.bg_color, light: self.light}
    }

}
//****************************************************

pub struct World{
    entities: Vec<Box<Entity>>,
    bg_color: Color,
    light: Light,
}

impl World{

    pub fn shoot_ray(&self, ray: &Ray) -> Color{
        let epsilon = 0.01;
        let mut t = f64::INFINITY;
        let mut h: Option<Hit> =None;
        for e in &self.entities{
            if let Some(hit) = e.as_ref().hit(ray){
                if hit.t < t && -hit.n.dot(&ray.dir) > 0.0{
                    t=hit.t;
                    h=Some(hit);
                }
            }

        }
        if let Some(hit) = h{
            //let ambient = Vector::new(0.4,0.4,-0.7).normalize();
            hit.c * self.get_radiance_from_light(&(ray.orig+ray.dir*(hit.t-epsilon)), &hit.n)
            //hit.c * (hit.n.dot(&-ambient).max(0.0)+self.get_radiance_from_light(&(ray.orig+ray.dir*(hit.t-epsilon)), &hit.n))
            //hit.c* (-hit.n.dot(&ray.dir)).max(0.0)
        }else{
            self.bg_color
        }
    }

    fn get_radiance_from_light(&self, p: &Vector, n:&Vector)-> f64{

        let samples_from_light = self.light.get_sample_points();
        let mut est:f64 = 0.0;
        for s in samples_from_light.iter(){
            let dir = p-s;
            let ray =Ray{orig:*s, dir};
            let dir = dir.normalize();
            if self.is_light_visible(&ray){
                 est += self.light.n.dot(&dir)*-n.dot(&dir)/ dir.norm_squared();
            }
        }
        (est*self.light.A*self.light.I/(PI)).max(0.0)
        /*let dir = self.light.get_sample_point()-p;
        let ray =Ray{orig:*p, dir};
        for e  in &self.entities{
            if let Some(hit) = e.as_ref().hit(&ray){
                if hit.t<1.0{
                    return None
                }
            }
        }
        Some((-dir).normalize())*/
    }

    fn is_light_visible(&self, ray:&Ray) ->bool{
        if ray.dir.dot(&self.light.n)< 0.0{
            return false;
        }
        for e in &self.entities{
            if let Some(hit) = e.as_ref().hit(&ray){
                if hit.t < 1.0 {
                    return false;
                }
            }
        }
        true
    }
}
//****************************************************

pub struct Light{
    orig: Vector,
    a: Vector,
    b: Vector,
    n: Vector,
    I: f64,
    A: f64
}

impl Light{
    pub fn new(orig:Vector, a:Vector, b:Vector, I:f64) -> Light{
        let cross = a.cross(&b);
        Light{orig, a, b, n:cross.normalize(), A:cross.norm(),I}
    }

    pub fn default() -> Light{

        Light::new(Vector::new(-0.2,0.9,-0.2), Vector::new(0.4,0.0,0.0),  Vector::new(0.0,0.0,0.4),3.0)
    }

    pub fn get_sample_points(&self) -> [Vector;4]{
        //TODO: better sampling
        let mut rng = SmallRng::from_entropy();

        [
            self.orig+self.a * rng.gen::<f64>()*0.5 + self.b * rng.gen::<f64>()*0.5,
            self.orig+self.a * (rng.gen::<f64>()*0.5+0.5) + self.b * rng.gen::<f64>()*0.5,
            self.orig+self.a * rng.gen::<f64>()*0.5 + self.b * (rng.gen::<f64>()*0.5+0.5),
            self.orig+self.a * (rng.gen::<f64>()*0.5+0.5) + self.b * (rng.gen::<f64>()*0.5+0.5),
        ]
        //[self.orig , self.orig+self.a, self.orig+self.b, self.orig+self.a+self.b]
    }
}


