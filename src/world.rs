use crate::types::*;
use crate::utils::Ray;

/*
builder!(WorldBuilder => World{
    entitys: Vec<u32> = Vec::new()
});*/

pub struct World{

}

pub struct WorldBuilder{

}

impl WorldBuilder{
    pub fn new() -> WorldBuilder{
        WorldBuilder{}
    }

    pub fn build(&self) -> World {
        World{}
    }

}




impl World{

    pub fn shoot_ray(&self, ray: &Ray) -> Color{
        Color{data:[200,20,210]}
    }
}
