use crate::{Bvh, BvhVolume};

use std::fmt::{Debug, Formatter, Result};

impl<Volume: BvhVolume, T: Copy + Debug> Debug for Bvh<Volume, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        writeln!(f)?;
        print_node(f, self, 0, 0)
    }
}

fn print_node<Volume: BvhVolume, T: Copy + Debug>(
    f: &mut Formatter<'_>,
    bvh: &Bvh<Volume, T>,
    index: u32,
    level: usize,
) -> Result {
    let node = &bvh.nodes[index as usize];
    write!(f, "|{} ", "-".repeat(level))?;
    if node.count == 0 {
        writeln!(f, "Node: {:?}", node.volume)?;
        print_node(f, bvh, node.start_index, level + 1)?;
        print_node(f, bvh, node.start_index + 1, level + 1)?;
    } else {
        writeln!(f, "Leaf: {:?}", node.volume)?;
        print_items(f, bvh, node.start_index, node.count, level + 1)?;
    }
    Ok(())
}

fn print_items<Volume: BvhVolume, T: Copy + Debug>(
    f: &mut Formatter<'_>,
    bvh: &Bvh<Volume, T>,
    index: u32,
    count: u32,
    level: usize,
) -> Result {
    for i in 0..count {
        let item = &bvh.items[(index + i) as usize];
        writeln!(
            f,
            "|{} Item: {:?} ({:?})",
            "-".repeat(level),
            item.t,
            item.volume
        )?;
    }
    Ok(())
}
