pub trait Container<R, H>: Send + Sync + Sized + 'static {
    fn closest_hit(&self, r: &R) -> Option<H>;
    fn any_hit<F: Fn(H) -> bool>(&self, r: &R, f: F) -> bool;
}

pub trait ContainerWithIter<R, H>: Container<R, H> {
    type IndexType;

    fn next(&self, ray: &R, i: &mut Self::IndexType) -> Option<H>;
    fn start_index(&self) -> Self::IndexType;

    fn hits<'c, 'r>(&'c self, r: &'r R) -> ContainerIterator<'c, 'r, R, H, Self> {
        ContainerIterator {
            container: self,
            i: self.start_index(),
            r,
        }
    }
}

pub trait ContainerCreator<R, H> {
    type Output: Container<R, H>;

    fn new() -> Self;
    fn create(self) -> Self::Output;
}

pub trait EntitiesAdder<E, R, H>: ContainerCreator<R, H> {
    fn add_entities(&mut self, v: Vec<E>);
}
pub struct ContainerIterator<'c, 'r, R, H, C: ContainerWithIter<R, H>> {
    container: &'c C,
    i: C::IndexType,
    r: &'r R,
}

impl<'c, 'r, R, H, C: ContainerWithIter<R, H>> Iterator for ContainerIterator<'c, 'r, R, H, C> {
    type Item = H;

    fn next(&mut self) -> Option<Self::Item> {
        self.container.next(&self.r, &mut self.i)
    }
}
