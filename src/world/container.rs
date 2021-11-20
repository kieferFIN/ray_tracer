use crate::world::entities::Entity;
use crate::{Hit, Ray};

pub trait Container: Send + Sync + 'static {
    fn hits(&self, r: &Ray) -> Vec<Hit>;
    fn hits_any_filter<F: FnMut(Hit) -> bool>(&self, r: &Ray, f: F) -> bool;
    fn hits_any(&self, r: &Ray) -> bool {
        self.hits_any_filter(r, |h| h.t < 1.0)
    }
}

pub trait ContainerCreator<E: Entity> {
    type Output: Container;

    fn create(entities: Vec<E>) -> Self::Output;
}

pub struct BasicContainer<E> {
    entities: Vec<E>,
}

impl<E: Entity> Container for BasicContainer<E> {
    fn hits(&self, r: &Ray) -> Vec<Hit> {
        self.entities.iter().filter_map(|t| t.hit(&r)).collect()
    }

    fn hits_any_filter<F: FnMut(Hit) -> bool>(&self, r: &Ray, mut f: F) -> bool {
        self.entities.iter().filter_map(|t| t.hit(&r)).any(|h| f(h))
    }
}

pub struct BasicCreator {}

impl<E: Entity> ContainerCreator<E> for BasicCreator {
    type Output = BasicContainer<E>;

    fn create(entities: Vec<E>) -> Self::Output {
        BasicContainer { entities }
    }
}
