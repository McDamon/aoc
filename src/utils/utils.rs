use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

pub fn get_lines(input_file: &str) -> Vec<String> {
    let path = Path::new(input_file);
    let display = path.display();

    let file = match File::open(input_file) {
        // The `description` method of `io::Error` returns a string that describes the error
        Err(why) => panic!(
            "couldn't open {}: {}",
            display,
            <dyn Error>::to_string(&why)
        ),
        Ok(file) => file,
    };

    let reader = BufReader::new(file);
    reader
        .lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Direction {
    N,
    S,
    E,
    W,
}

#[derive(Debug, Default, Clone, Copy, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub enum Compass {
    #[default]
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

pub struct ListNode<T> {
    pub val: T,
    pub next: Option<Box<ListNode<T>>>,
}

impl<T> ListNode<T> {
    pub fn new(val: T) -> ListNode<T> {
        ListNode { val, next: None }
    }
}

#[derive(Debug)]
pub struct TreeNode<T>
where
    T: PartialEq,
{
    pub idx: usize,
    pub val: T,
    pub parent: Option<usize>,
    pub children: Vec<usize>,
}

impl<T> TreeNode<T>
where
    T: PartialEq,
{
    pub fn new(idx: usize, val: T) -> Self {
        Self {
            idx,
            val,
            parent: None,
            children: vec![],
        }
    }
}

#[derive(Debug, Default)]
pub struct ArenaTree<T>
where
    T: PartialEq,
{
    pub arena: Vec<TreeNode<T>>,
}

impl<T> ArenaTree<T>
where
    T: PartialEq,
{
    pub fn add_node(&mut self, val: T) -> usize {
        // First see if it exists
        for node in &self.arena {
            if node.val == val {
                return node.idx;
            }
        }
        // Otherwise, add new node
        let idx = self.arena.len();
        self.arena.push(TreeNode::new(idx, val));
        idx
    }

    pub fn size(&self) -> usize {
        self.arena.len()
    }

    pub fn edges(&self) -> usize {
        self.arena
            .iter()
            .fold(0, |acc, node| acc + node.children.len())
    }

    pub fn depth(&self, idx: usize) -> usize {
        match self.arena[idx].parent {
            Some(id) => 1 + self.depth(id),
            None => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arena_tree() {
        let mut tree: ArenaTree<u32> = ArenaTree::default();

        let tree_node_1 = tree.add_node(1);
        let tree_node_2 = tree.add_node(2);
        let tree_node_3 = tree.add_node(3);
        let tree_node_4 = tree.add_node(4);
        let tree_node_5 = tree.add_node(5);

        tree.arena[tree_node_1].children.push(tree_node_2);
        tree.arena[tree_node_2].parent = Some(tree_node_1);
        tree.arena[tree_node_2].children.push(tree_node_3);
        tree.arena[tree_node_3].parent = Some(tree_node_2);
        tree.arena[tree_node_3].children.push(tree_node_4);
        tree.arena[tree_node_4].parent = Some(tree_node_3);
        tree.arena[tree_node_4].children.push(tree_node_5);
        tree.arena[tree_node_5].parent = Some(tree_node_4);

        assert_eq!(tree.size(), 5);
        assert_eq!(tree.edges(), 4);
        assert_eq!(tree.depth(tree_node_5), 4);
    }
}
