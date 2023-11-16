use crate::{Aabb, BvhNode, Dim};

const SEARCH_RADIUS: usize = 14;

#[derive(Default)]
pub struct FindCache([[f32; SEARCH_RADIUS]; SEARCH_RADIUS]);

impl FindCache {
    #[inline(always)]
    fn back(&self, index: usize, other: usize) -> f32 {
        self.0[other % SEARCH_RADIUS][index % SEARCH_RADIUS]
    }

    #[inline(always)]
    fn front(&mut self, index: usize, other: usize) -> &mut f32 {
        &mut self.0[index % SEARCH_RADIUS][other % SEARCH_RADIUS]
    }
}

pub fn find_best_node<D: Dim>(cache: &mut FindCache, index: usize, nodes: &[BvhNode<D>]) -> usize {
    let mut best_node = index;
    let mut best_area = f32::INFINITY;

    let begin = index - SEARCH_RADIUS.min(index);
    for other in (begin..=index).rev().skip(1) {
        let area = cache.back(index, other);
        if area < best_area {
            best_node = other;
            best_area = area;
        }
    }

    let our_aabb = nodes[index].aabb;
    let end = index + SEARCH_RADIUS + 1;
    for (other, node) in nodes.iter().enumerate().take(end).skip(index + 1) {
        let area = our_aabb.merge(&node.aabb).area();
        *cache.front(index, other) = area;
        if area < best_area {
            best_node = other;
            best_area = area;
        }
    }
    best_node
}

// TODO: Test find_best_node
