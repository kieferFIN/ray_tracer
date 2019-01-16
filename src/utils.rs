
use crate::types::*;

pub struct Ray{
    pub orig: Vector,
    pub dir: Vector,


}

impl Ray{
    pub fn new(orig: Vector, dir: Vector) ->Ray{

        Ray{orig, dir: dir.normalize()}
    }

    pub fn look_at(orig: Vector, dest: Vector) -> Ray{
        Ray::new(orig, dest-orig)
    }
}