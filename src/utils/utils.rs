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

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, Ord, PartialOrd)]
pub enum Direction {
    N,
    S,
    E,
    W,
}

impl Direction {
    pub fn to_delta(self) -> (isize, isize) {
        match self {
            Direction::E => (1, 0),
            Direction::S => (0, 1),
            Direction::W => (-1, 0),
            Direction::N => (0, -1),
        }
    }

    pub fn turn_left(&self) -> Direction {
        match self {
            Direction::E => Direction::N,
            Direction::S => Direction::E,
            Direction::W => Direction::S,
            Direction::N => Direction::W,
        }
    }

    pub fn turn_right(&self) -> Direction {
        match self {
            Direction::E => Direction::S,
            Direction::S => Direction::W,
            Direction::W => Direction::N,
            Direction::N => Direction::E,
        }
    }

    pub fn all() -> impl Iterator<Item = Direction> {
        [Direction::E, Direction::S, Direction::W, Direction::N].iter().copied()
    }
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
    T: PartialEq + Clone,
{
    pub idx: usize,
    pub val: T,
    pub parent: Option<usize>,
    pub children: Vec<usize>,
}

impl<T> TreeNode<T>
where
    T: PartialEq + Clone,
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
    T: PartialEq + Clone,
{
    pub arena: Vec<TreeNode<T>>,
}

impl<T> ArenaTree<T>
where
    T: PartialEq + Clone,
{
    pub fn get_node(&self, idx: usize) -> Option<T> {
        self.arena.get(idx).map(|node| node.val.clone())
    }

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

    pub fn idx(&self, val: T) -> Option<usize> {
        for node in &self.arena {
            if node.val == val {
                return Some(node.idx);
            }
        }
        None
    }

    pub fn clear(&mut self) {
        self.arena.clear();
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
