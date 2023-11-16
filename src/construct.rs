use crate::search::{find_best_node, FindCache};
use crate::{Aabb, Bvh, BvhItem, BvhNode, Dim, Pos};

use std::collections::VecDeque;

const TRAVERSE_COST: f32 = 1.5;

impl<D: Dim, T: Copy + std::fmt::Debug> Bvh<D, T> {
    /// Construct a BVH from a size and iterator
    pub fn new(n: usize, iter: impl IntoIterator<Item = (T, impl Into<D::Aabb>)>) -> Self {
        if n == 0 {
            return Self::default();
        }

        let mut current_nodes = Vec::with_capacity(n);
        let mut items = Vec::with_capacity(n);

        for (i, (t, aabb)) in iter.into_iter().enumerate() {
            let aabb = aabb.into();
            current_nodes.push(BvhNode {
                aabb,
                count: 1,
                start_index: i as u32,
            });
            items.push(BvhItem { aabb, t });
        }

        radsort::sort_by_cached_key(&mut current_nodes, |node: &BvhNode<D>| {
            node.aabb.center().code()
        });

        let mut nodes = vec![
            BvhNode {
                aabb: D::Aabb::INFINITY,
                count: 0,
                start_index: u32::MAX,
            };
            2 * n - 1
        ];
        let mut insert_index = nodes.capacity();

        let mut next_nodes = Vec::with_capacity(n);
        let mut merge = Vec::with_capacity(n);

        let mut find_cache = FindCache::default();
        // Create parent nodes for the best combinations of nodes until we have just 1 parent node
        while current_nodes.len() > 1 {
            for index in 0..current_nodes.len() {
                let best = find_best_node(&mut find_cache, index, &current_nodes);
                merge.push(best)
            }

            for (index, best) in merge.iter().enumerate() {
                if merge[*best] != index {
                    next_nodes.push(current_nodes[index]);
                    continue;
                }

                if *best > index {
                    continue;
                }

                let left = &current_nodes[index];
                let right = &current_nodes[*best];
                let parent_aabb = left.aabb.merge(&right.aabb);

                insert_index -= 2;
                nodes[insert_index] = current_nodes[index];
                nodes[insert_index + 1] = current_nodes[*best];
                next_nodes.push(BvhNode {
                    aabb: parent_aabb,
                    count: 0,
                    start_index: insert_index as u32,
                });
            }

            (next_nodes, current_nodes) = (current_nodes, next_nodes);
            next_nodes.clear();
            merge.clear();
        }

        insert_index -= 1;
        nodes[insert_index] = current_nodes[0];

        // TODO: We shouldn't make a new vec of this yet
        let mut nodes = nodes[insert_index..].to_vec();

        // Order the list of items to match the nodes
        let unordered_items = items;
        let mut items = Vec::with_capacity(n);
        let mut stack = VecDeque::with_capacity((n as f32).log2().ceil() as usize + 10);
        stack.push_back(0u32);
        while let Some(index) = stack.pop_front() {
            let node = &mut nodes[index as usize];
            if node.count == 0 {
                stack.push_front(node.start_index + 1);
                stack.push_front(node.start_index);
                continue;
            }

            items.push(unordered_items[node.start_index as usize]);
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
                if ((left.count + right.count) as f32 - TRAVERSE_COST) * parent.aabb.area()
                    < left.count as f32 * left.aabb.area() + right.count as f32 * right.aabb.area()
                {
                    let start_index = parent.start_index;

                    // Merge the leaves
                    nodes[index as usize] = BvhNode {
                        aabb: parent.aabb,
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
use crate::dim2::{Bvh2d, Vec2};

#[test]
fn test_bvh_new() {
    let items = vec![
        (1, (Vec2::ONE, Vec2::splat(2.))),
        (2, (Vec2::splat(2.), Vec2::splat(3.))),
        (3, (Vec2::splat(0.9), Vec2::splat(1.9))),
        (4, (Vec2::splat(0.2), Vec2::splat(2.2))),
        (5, (Vec2::ONE, Vec2::splat(5.))),
    ];

    let bvh = Bvh2d::new(items.len(), items);
    // The number of items should match the input
    assert_eq!(bvh.items.len(), 5);
    // Some of the nodes should have gotten merged
    assert!(bvh.items.len() < 5 * 2 - 1);
}
