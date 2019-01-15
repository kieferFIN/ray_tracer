use crate::types::*;
use crate::utils::Ray;


pub struct World{

}

impl World{
    pub fn new() -> World{
        World{}
    }

    pub fn shoot_ray(&self, ray: &Ray) -> Color{
        Color{data:[200,20,210]}
    }
}
