use crate::types::*;
use crate::utils::Ray;

use std::f64;
use nalgebra::Matrix3;
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
pub struct Ball {
    pub c: Color,
    pub r: f64,
    pub o: Vector
}

impl Entity for Ball{
    fn hit(&self, ray: &Ray) -> Option<(f64,Color)> {
        let a = ray.orig - self.o;
        let b = 2.0*ray.dir.dot(&a);
        let d = b.powi(2) - 4.0 * (a.dot(&a)-self.r.powi(2)) ;
        if d >= 0.0 {
            let d_sqrt = d.sqrt();
            let t = 0.0_f64.max((-b +d_sqrt)*0.5).min(0.0_f64.max((-b -d_sqrt)*0.5));
            if t>0.0{
                return Some((t ,self.c));
            }
        }
        None

    }
}

pub struct TriangleBuilder{
    v: Vec<Vector>,
    n: Vec<Vector>,
    c: Color
}

impl TriangleBuilder{
    pub fn new(c: Color) -> TriangleBuilder{
        TriangleBuilder{v:Vec::new(), n:Vec::new(), c}
    }
    pub fn add(&mut self, coord:[f64;3], normal:[f64;3]){
        self.v.push(Vector::new(coord[0],coord[1],coord[2]));
        self.n.push(Vector::new(normal[0],normal[1],normal[2]));

    }
    pub fn build(&self) ->Triangle{
        let vertices = [self.v[0],self.v[1],self.v[2]];
        let normals = [self.n[0],self.n[1],self.n[2]];
        let a_b = self.v[0]-self.v[1];
        let a_c = self.v[0]-self.v[2];
        Triangle{vertices, normals, color:self.c, a_b, a_c}
    }

}

pub struct Triangle{
    color: Color,
    vertices: [Vector;3],
    normals: [Vector;3],
    pub a_b: Vector,
    pub a_c: Vector

}

impl Entity for Triangle{
    fn hit(&self, ray: &Ray) -> Option<(f64, Color)> {
        let m = Matrix3::from_columns(&[self.a_b, self.a_c, ray.dir]);
        let decomp = m.lu();
        let b = self.vertices[0]-ray.orig;
        let x = decomp.solve(&b).expect("Linear resolution failed.");
        //println!("{} {}", x.x, x.y);
        if x.x >= 0.0 && x.y >= 0.0 && x.x + x.y <1.0 {
            //println!("*");
            return Some((x.z, self.color))
        }

        None
    }

}

