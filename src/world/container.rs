use crate::{Hit, Ray};

pub trait Container: Send + Sync + Sized + 'static {
    fn closest_hit(&self, r: &Ray) -> Option<Hit>;
    fn any_hit<F: Fn(Hit) -> bool>(&self, r: &Ray, f: F) -> bool;
}

pub trait ContainerWithIter: Container {
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

pub trait ContainerCreator {
    type Output: Container;

    fn new() -> Self;
    fn create(self) -> Self::Output;
}

pub trait EntitiesAdder<E>: ContainerCreator {
    fn add_entities(&mut self, v: Vec<E>);
}
pub struct ContainerIterator<'c, 'r, C: ContainerWithIter> {
    container: &'c C,
    i: C::IndexType,
    r: &'r Ray,
}

impl<'c, 'r, C: ContainerWithIter> Iterator for ContainerIterator<'c, 'r, C> {
    type Item = Hit;

    fn next(&mut self) -> Option<Self::Item> {
        self.container.next(&self.r, &mut self.i)
    }
}
