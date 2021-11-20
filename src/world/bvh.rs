use crate::Vector;

struct BVH {
    nodes: Vec<BvhNode>,
}

impl BVH {
    fn create<T: AabbCreator>(e: &Vec<T>) -> BVH {
        BVH {
            nodes: Vec::with_capacity(e.len() * 2),
        }
    }
}

trait AabbCreator {
    fn create_aabb(&self) -> AABB;
}

enum BvhNode {
    Node { left_aabb: AABB, right_aabb: AABB },
    Leaf { aabb: AABB, index: usize },
}

struct AABB {
    min: Vector,
    max: Vector,
}
