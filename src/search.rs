use crate::{BvhNode, BvhVolume};

const SEARCH_RADIUS: usize = 14;

mod cache {
    use super::SEARCH_RADIUS;

    #[derive(Clone, Copy)]
    pub struct ModIndex(usize);

    impl ModIndex {
        pub fn new(index: usize) -> Self {
            Self(index % SEARCH_RADIUS)
        }
    }

    #[derive(Default)]
    pub struct FindCache([[f32; SEARCH_RADIUS]; SEARCH_RADIUS]);

    impl FindCache {
        #[inline(always)]
        pub fn back(&self, mod_index: ModIndex, other: usize) -> f32 {
            *unsafe {
                self.0
                    .get_unchecked(other % SEARCH_RADIUS)
                    .get_unchecked(mod_index.0)
            }
        }

        #[inline(always)]
        pub fn set_front(&mut self, mod_index: ModIndex, other: usize, value: f32) {
            *unsafe {
                self.0
                    .get_unchecked_mut(mod_index.0)
                    .get_unchecked_mut(other % SEARCH_RADIUS)
            } = value;
        }
    }
}
pub use cache::FindCache;
use cache::*;

#[inline(always)]
pub fn find_best_node<Volume: BvhVolume>(
    cache: &mut FindCache,
    index: usize,
    nodes: &[BvhNode<Volume>],
) -> usize {
    let mut best_node = index;
    let mut best_area = f32::INFINITY;

    let mod_index = ModIndex::new(index);

    let begin = index.saturating_sub(SEARCH_RADIUS);
    for other in begin..index {
        let area = cache.back(mod_index, other);
        if area < best_area {
            best_node = other;
            best_area = area;
        }
    }

    let our_aabb = &nodes[index].volume;
    let end = index + SEARCH_RADIUS + 1;
    for (other, node) in nodes.iter().enumerate().take(end).skip(index + 1) {
        let area = our_aabb.merge(&node.volume).visible_area();
        cache.set_front(mod_index, other, area);
        if area < best_area {
            best_node = other;
            best_area = area;
        }
    }
    best_node
}

// TODO: Test find_best_node
