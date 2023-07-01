use crate::world::container::EntitiesAdder;
use crate::world::entities::{Entity, Triangle};
use crate::world::{Container, ContainerCreator};
use crate::{Hit, Ray, ToVector, Vector};

struct BVH {
    nodes: Vec<BvhNode>,
}

impl BVH {
    fn create<T: Into<AABB>>(e: &Vec<T>) -> BVH {
        BVH {
            nodes: Vec::with_capacity(e.len() * 2),
        }
    }
}

enum BvhNode {
    Node { left_aabb: AABB, right_aabb: AABB },
    Leaf { aabb: AABB, index: usize },
}

pub struct AABB {
    min: Vector,
    max: Vector,
}

impl AABB {
    fn intersect(&self, r: &Ray) -> Option<(f64, f64)> {
        let (vmin, vmax) = (self.min - r.orig)
            .component_div(&r.dir)
            .inf_sup(&(self.max - r.orig).component_div(&r.dir));
        let v = (vmin.max(), vmax.min());
        if v.0 > v.1 || v.1 < 0.0 {
            None
        } else {
            Some(v)
        }
    }
}

impl ToAABB for Triangle {
    fn to_aabb(&self) -> AABB {
        AABB {
            min: (
                self.vertices[0][0]
                    .min(self.vertices[1][0])
                    .min(self.vertices[2][0]),
                self.vertices[0][1]
                    .min(self.vertices[1][1])
                    .min(self.vertices[2][1]),
                self.vertices[0][2]
                    .min(self.vertices[1][2])
                    .min(self.vertices[2][2]),
            )
                .to_vector(),
            max: (
                self.vertices[0][0]
                    .max(self.vertices[1][0])
                    .max(self.vertices[2][0]),
                self.vertices[0][1]
                    .max(self.vertices[1][1])
                    .max(self.vertices[2][1]),
                self.vertices[0][2]
                    .max(self.vertices[1][2])
                    .max(self.vertices[2][2]),
            )
                .to_vector(),
        }
    }
}

pub trait ToAABB {
    fn to_aabb(&self) -> AABB;
}

//**************************************

pub struct SimpleBVH<E> {
    entities: Vec<E>,
    nodes: Vec<(AABB, usize)>,
}

pub struct SimpleBVHCreator<E> {
    entities: Vec<E>,
}

impl<E: Entity> Container<Ray, Hit> for SimpleBVH<E> {
    fn closest_hit(&self, r: &Ray) -> Option<Hit> {
        let mut hit: Option<Hit> = None;
        let mut t = f64::INFINITY;
        for (aabb, i) in &self.nodes {
            if let Some((tmin, _tmax)) = aabb.intersect(r) {
                if tmin < t {
                    if let Some(h) = self.entities[*i].hit(r) {
                        if h.t < t {
                            t = h.t;
                            hit = Some(h);
                        }
                    }
                }
            }
        }
        hit
    }

    fn any_hit<F: Fn(Hit) -> bool>(&self, r: &Ray, f: F) -> bool {
        for (aabb, i) in &self.nodes {
            if let Some((tmin, _tmax)) = aabb.intersect(r) {
                if tmin < 1.0 {
                    if let Some(h) = self.entities[*i].hit(r) {
                        if f(h) {
                            return true;
                        }
                    }
                }
            }
        }
        false
    }
}

impl<E: Entity + ToAABB> ContainerCreator<Ray, Hit> for SimpleBVHCreator<E> {
    type Output = SimpleBVH<E>;

    fn new() -> Self {
        Self { entities: vec![] }
    }

    fn create(self) -> Self::Output {
        let nodes = self
            .entities
            .iter()
            .enumerate()
            .map(|(i, e)| (e.to_aabb(), i))
            .collect();
        SimpleBVH {
            entities: self.entities,
            nodes,
        }
    }
}

impl<E: Entity + ToAABB> EntitiesAdder<E, Ray, Hit> for SimpleBVHCreator<E> {
    fn add_entities(&mut self, v: Vec<E>) {
        self.entities = v;
    }
}
