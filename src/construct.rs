use crate::search::{find_best_node, FindCache};
use crate::{Bvh, BvhItem, BvhNode, BvhVolume};

use std::collections::VecDeque;

const TRAVERSE_COST: f32 = 1.5;

impl<Volume: BvhVolume, T: Copy + std::fmt::Debug> Bvh<Volume, T> {
    /// Construct a BVH from a size and iterator
    pub fn new(max_items: usize, iter: impl IntoIterator<Item = (T, impl Into<Volume>)>) -> Self {
        if max_items == 0 {
            return Self::default();
        }

        let mut current_nodes = Vec::with_capacity(max_items);
        let mut items = Vec::with_capacity(max_items);

        for (i, (t, volume)) in iter.into_iter().enumerate() {
            let volume = volume.into();
            current_nodes.push((
                BvhNode {
                    volume: volume.clone(),
                    count: 1,
                    start_index: i as u32,
                },
                volume.morton_code(),
            ));
            items.push(BvhItem { volume, t });
        }
        let n_items = items.len();

        radsort::sort_by_key(&mut current_nodes, |(_, code)| *code);
        let mut current_nodes = current_nodes
            .drain(..)
            .map(|(node, _)| node)
            .collect::<Vec<_>>();

        let mut nodes = vec![
            BvhNode {
                volume: Volume::INFINITY,
                count: 0,
                start_index: u32::MAX,
            };
            2 * n_items - 1
        ];
        let mut insert_index = nodes.capacity();

        let mut next_nodes = Vec::with_capacity(n_items);
        let mut merge = Vec::with_capacity(n_items);

        let mut find_cache = FindCache::default();
        // Create parent nodes for the best combinations of nodes until we have just 1 parent node
        while current_nodes.len() > 1 {
            for index in 0..current_nodes.len() {
                let best = find_best_node(&mut find_cache, index, &current_nodes);
                merge.push(best)
            }

            for (index, best) in merge.iter().enumerate() {
                if merge[*best] != index {
                    next_nodes.push(current_nodes[index].clone());
                    continue;
                }

                if *best > index {
                    continue;
                }

                let left = &current_nodes[index];
                let right = &current_nodes[*best];
                let parent_aabb = left.volume.merge(&right.volume);

                insert_index -= 2;
                nodes[insert_index] = left.clone();
                nodes[insert_index + 1] = right.clone();
                next_nodes.push(BvhNode {
                    volume: parent_aabb,
                    count: 0,
                    start_index: insert_index as u32,
                });
            }

            (next_nodes, current_nodes) = (current_nodes, next_nodes);
            next_nodes.clear();
            merge.clear();
        }

        insert_index -= 1;
        nodes[insert_index] = current_nodes[0].clone();

        debug_assert_eq!(insert_index, 0);

        // Order the list of items to match the nodes
        let unordered_items = items;
        let mut items = Vec::with_capacity(n_items);
        let mut stack = VecDeque::with_capacity((n_items as f32).log2().ceil() as usize + 10);
        stack.push_back(0u32);
        while let Some(index) = stack.pop_front() {
            let node = &mut nodes[index as usize];
            if node.count == 0 {
                stack.push_front(node.start_index + 1);
                stack.push_front(node.start_index);
                continue;
            }

            items.push(unordered_items[node.start_index as usize].clone());
            node.start_index = items.len() as u32 - 1;
        }

        // Merge leaves according to the Surface Area Heuristic
        let mut can_merge = true;
        while can_merge {
            can_merge = false;

            stack.push_back(0u32);
            while let Some(index) = stack.pop_front() {
                let parent = &nodes[index as usize];

                if parent.count != 0 || parent.start_index == 0 {
                    // Leaf nodes and dead nodes have nothing to merge
                    continue;
                }

                let left = &nodes[parent.start_index as usize];
                let right = &nodes[(parent.start_index + 1) as usize];
                if left.count == 0 || right.count == 0 {
                    if right.count == 0 {
                        stack.push_front(parent.start_index + 1);
                    }
                    if left.count == 0 {
                        stack.push_front(parent.start_index);
                    }
                    continue;
                }

                // Check the Surface Area Heuristic
                if ((left.count + right.count) as f32 - TRAVERSE_COST)
                    * parent.volume.visible_area()
                    < left.count as f32 * left.volume.visible_area()
                        + right.count as f32 * right.volume.visible_area()
                {
                    let start_index = parent.start_index;

                    // Merge the leaves
                    nodes[index as usize] = BvhNode {
                        volume: parent.volume.clone(),
                        count: left.count + right.count,
                        start_index: left.start_index.min(right.start_index),
                    };

                    // Make sure the old nodes can get picked up as dead
                    nodes[start_index as usize].count = 0;
                    nodes[start_index as usize].start_index = 0;
                    nodes[(start_index + 1) as usize].count = 0;
                    nodes[(start_index + 1) as usize].start_index = 0;

                    // Since we merged nodes, we can do another pass
                    can_merge = true;
                }
            }
        }

        // TODO: Clean up dead nodes (count = 0, start_index = 0)

        Self { nodes, items }
    }
}

#[cfg(test)]
use crate::dim2::{BvhAabb2d, Vec2};
#[cfg(test)]
use bevy_math::bounding::Aabb2d;

#[test]
fn test_bvh_new() {
    let items = vec![
        (1, Aabb2d::new(Vec2::ONE, Vec2::splat(2.))),
        (2, Aabb2d::new(Vec2::splat(2.), Vec2::splat(3.))),
        (3, Aabb2d::new(Vec2::splat(0.9), Vec2::splat(1.9))),
        (4, Aabb2d::new(Vec2::splat(0.2), Vec2::splat(2.2))),
        (5, Aabb2d::new(Vec2::ONE, Vec2::splat(5.))),
    ];

    let bvh = BvhAabb2d::new(items.len(), items);
    // The number of items should match the input
    assert_eq!(bvh.items.len(), 5);
    // Some of the nodes should have gotten merged
    assert!(bvh.items.len() < 5 * 2 - 1);
}
