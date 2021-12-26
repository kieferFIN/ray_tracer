use crate::world::container::{ContainerWithIter, EntitiesAdder};
use crate::world::entities::Entity;
use crate::world::{Container, ContainerCreator};
use crate::{Hit, Ray};

pub struct BasicContainer<E> {
    entities: Vec<E>,
}

impl<E: Entity> Container for BasicContainer<E> {
    fn closest_hit(&self, r: &Ray) -> Option<Hit> {
        self.entities
            .iter()
            .filter_map(|e| e.hit(&r))
            .min_by(|h1, h2| h1.partial_cmp(h2).unwrap())
    }

    fn any_hit<F: Fn(Hit) -> bool>(&self, r: &Ray, f: F) -> bool {
        self.entities.iter().filter_map(|e| e.hit(r)).any(f)
    }
}

impl<E: Entity> ContainerWithIter for BasicContainer<E> {
    type IndexType = usize;

    fn next(&self, ray: &Ray, i: &mut usize) -> Option<Hit> {
        loop {
            if *i >= self.entities.len() {
                return None;
            } else {
                *i += 1;
                if let Some(h) = self.entities[*i - 1].hit(ray) {
                    return Some(h);
                }
            }
        }
    }

    fn start_index(&self) -> usize {
        0
    }
}

pub struct BasicCreator<E: Entity> {
    entities: Vec<E>,
}

impl<E: Entity> ContainerCreator for BasicCreator<E> {
    type Output = BasicContainer<E>;

    fn new() -> Self {
        BasicCreator { entities: vec![] }
    }

    fn create(self) -> Self::Output {
        BasicContainer {
            entities: self.entities,
        }
    }
}

impl<E: Entity> EntitiesAdder<E> for BasicCreator<E> {
    fn add_entities(&mut self, v: Vec<E>) {
        self.entities = v;
    }
}
