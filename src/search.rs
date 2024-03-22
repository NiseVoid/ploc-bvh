use crate::{BvhNode, BvhVolume};

const SEARCH_RADIUS: usize = 14;

#[derive(Default)]
pub struct FindCache([[f32; SEARCH_RADIUS]; SEARCH_RADIUS]);

impl FindCache {
    #[inline(always)]
    unsafe fn back(&self, mod_index: usize, other: usize) -> f32 {
        *self
            .0
            .get_unchecked(other % SEARCH_RADIUS)
            .get_unchecked(mod_index)
    }

    #[inline(always)]
    unsafe fn set_front(&mut self, mod_index: usize, other: usize, value: f32) {
        *self
            .0
            .get_unchecked_mut(mod_index)
            .get_unchecked_mut(other % SEARCH_RADIUS) = value;
    }
}

#[inline(always)]
pub fn find_best_node<Volume: BvhVolume>(
    cache: &mut FindCache,
    index: usize,
    nodes: &[BvhNode<Volume>],
) -> usize {
    let mut best_node = index;
    let mut best_area = f32::INFINITY;

    let mod_index = index % SEARCH_RADIUS;

    let begin = index.saturating_sub(SEARCH_RADIUS);
    for other in begin..index {
        // SAFETY: We pass in the modulo'd index
        let area = unsafe { cache.back(mod_index, other) };
        if area < best_area {
            best_node = other;
            best_area = area;
        }
    }

    let our_aabb = &nodes[index].volume;
    let end = index + SEARCH_RADIUS + 1;
    for (other, node) in nodes.iter().enumerate().take(end).skip(index + 1) {
        let area = our_aabb.merge(&node.volume).visible_area();
        // SAFETY: We pass in the modulo'd index
        unsafe {
            cache.set_front(mod_index, other, area);
        }
        if area < best_area {
            best_node = other;
            best_area = area;
        }
    }
    best_node
}

// TODO: Test find_best_node
