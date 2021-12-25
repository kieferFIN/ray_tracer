use crate::world::entities::Entity;
use crate::world::{Container, ContainerCreator};
use crate::{Hit, Ray};

pub struct BasicContainer<E> {
    entities: Vec<E>,
}

impl<E: Entity> Container for BasicContainer<E> {
    type IndexType = usize;

    fn next(&self, ray: &Ray, i: &mut usize) -> Option<Hit> {
        let mut j = *i;
        loop {
            if j >= self.entities.len() {
                return None;
            } else {
                j += 1;
                if let Some(h) = self.entities[j - 1].hit(ray) {
                    *i = j;
                    return Some(h);
                }
            }
        }
    }

    fn start_index(&self) -> usize {
        0
    }
}

pub struct BasicCreator {}

impl<E: Entity> ContainerCreator<E> for BasicCreator {
    type Output = BasicContainer<E>;

    fn create(entities: Vec<E>) -> Self::Output {
        BasicContainer { entities }
    }
}
