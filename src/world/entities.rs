use crate::types::*;

use nalgebra::Matrix3;

pub trait Entity: Sync + Send + 'static {
    fn hit(&self, ray: &Ray) -> Option<Hit>;
}
/*
pub struct TestBall {
    pub c: Color,
    pub r: f64,
    pub o: Vector
}

impl Entity for TestBall{
    fn hit(&self, ray: &Ray) -> Option<(Hit)> {
        let a = ray.orig - self.o;
        let b = 2.0*ray.dir.dot(&a);
        let d = b.powi(2) - 4.0 * (a.dot(&a)-self.r.powi(2)) ;
        if d >= 0.0 {
            let d_sqrt = d.sqrt();
            let t = 0.0_f64.max((-b +d_sqrt)*0.5).min(0.0_f64.max((-b -d_sqrt)*0.5));
            if t>0.0{
                let n = (ray.orig+ray.dir*t-self.o).normalize();
                return Some(Hit{t,n,c:self.c});

            }
        }
        None

    }
}*/
// ********************************************************

pub struct TriangleBuilder {
    v: Vec<Vector>,
    n: Vec<Vector>,
    c: Color,
}

impl TriangleBuilder {
    pub fn new(c: Color) -> TriangleBuilder {
        TriangleBuilder {
            v: Vec::new(),
            n: Vec::new(),
            c,
        }
    }
    pub fn add(&mut self, coord: [f64; 3], normal: [f64; 3]) {
        self.v.push(Vector::new(coord[0], coord[1], coord[2]));
        self.n.push(Vector::new(normal[0], normal[1], normal[2]));
    }
    pub fn build(&self) -> Triangle {
        let vertices = [self.v[0], self.v[1], self.v[2]];
        let normals = [
            self.n[0].normalize(),
            self.n[1].normalize(),
            self.n[2].normalize(),
        ];
        let a_b = self.v[0] - self.v[1];
        let a_c = self.v[0] - self.v[2];
        Triangle {
            vertices,
            normals,
            color: self.c,
            a_b,
            a_c,
        }
    }
}

pub struct Triangle {
    pub color: Color,
    pub vertices: [Vector; 3],
    pub normals: [Vector; 3],
    pub a_b: Vector,
    pub a_c: Vector,
}

impl Entity for Triangle {
    fn hit(&self, ray: &Ray) -> Option<Hit> {
        const EPSILON: f64 = 0.01;
        let m = Matrix3::from_columns(&[self.a_b, self.a_c, ray.dir]);
        let decomp = m.lu();
        let b = self.vertices[0] - ray.orig;
        let x = match decomp.solve(&b) {
            Some(x) => x,
            None => return None,
        };
        let beta = x.x;
        let gamma = x.y;
        let t = x.z;
        let alpha = 1.0 - beta - gamma;
        //println!("{} {}", x.x, x.y);
        if beta < 0.0 || gamma < 0.0 || alpha < 0.0 {
            return None;
        }
        let n = (self.normals[0] * alpha + self.normals[1] * beta + self.normals[2] * gamma)
            .normalize();
        if t > 0.0 && n.dot(&ray.dir) < 0.0 {
            Some(Hit {
                t,
                n,
                c: self.color,
                p: ray.orig + ray.dir * (t - EPSILON),
            })
        } else {
            None
        }
    }
}
// ********************************************************
