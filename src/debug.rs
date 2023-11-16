use crate::{Bvh, Dim};

use std::fmt::{Debug, Formatter, Result};

impl<D: Dim, T: Copy + Debug> Debug for Bvh<D, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        writeln!(f)?;
        print_node(f, self, 0, 0)
    }
}

fn print_node<D: Dim, T: Copy + Debug>(
    f: &mut Formatter<'_>,
    bvh: &Bvh<D, T>,
    index: u32,
    level: usize,
) -> Result {
    let node = bvh.nodes[index as usize];
    write!(f, "|{} ", "-".repeat(level))?;
    if node.count == 0 {
        writeln!(f, "Node: {:?}", node.aabb)?;
        print_node(f, bvh, node.start_index, level + 1)?;
        print_node(f, bvh, node.start_index + 1, level + 1)?;
    } else {
        writeln!(f, "Leaf: {:?}", node.aabb)?;
        print_items(f, bvh, node.start_index, node.count, level + 1)?;
    }
    Ok(())
}

fn print_items<D: Dim, T: Copy + Debug>(
    f: &mut Formatter<'_>,
    bvh: &Bvh<D, T>,
    index: u32,
    count: u32,
    level: usize,
) -> Result {
    for i in 0..count {
        let item = &bvh.items[(index + i) as usize];
        writeln!(f, "|{} Item: {:?} ({:?})", "-".repeat(level), item.t, item.aabb)?;
    }
    Ok(())
}
