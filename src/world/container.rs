use crate::world::entities::Entity;
use crate::{Hit, Ray};

pub trait Container: Send + Sync + Sized + 'static {
    type IndexType;
    fn next(&self, ray: &Ray, i: &mut Self::IndexType) -> Option<Hit>;
    fn start_index(&self) -> Self::IndexType;

    fn hits<'c, 'r>(&'c self, r: &'r Ray) -> ContainerIterator<'c, 'r, Self> {
        ContainerIterator {
            container: self,
            i: self.start_index(),
            r,
        }
    }
}

pub trait ContainerCreator<E: Entity> {
    type Output: Container;

    fn create(entities: Vec<E>) -> Self::Output;
}
pub struct ContainerIterator<'c, 'r, C: Container> {
    container: &'c C,
    i: C::IndexType,
    r: &'r Ray,
}

impl<'c, 'r, C: Container> Iterator for ContainerIterator<'c, 'r, C> {
    type Item = Hit;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = self.container.next(&self.r, &mut self.i);
        ret
    }
}

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
