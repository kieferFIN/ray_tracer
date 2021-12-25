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
