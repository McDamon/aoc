use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use hashbrown::HashMap;
use petgraph::graph::NodeIndex;

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

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, Default)]
pub enum MoveDir {
    #[default]
    Up,
    Down,
    Left,
    Right,
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
        [Direction::E, Direction::S, Direction::W, Direction::N]
            .iter()
            .copied()
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

pub fn get_all_paths<T>(
    graph: &petgraph::Graph<T, f64>,
    node_costs: &HashMap<NodeIndex, f64>,
    start_idx: NodeIndex,
    end_idx: NodeIndex,
) -> Vec<Vec<NodeIndex>> {
    let mut parents: HashMap<NodeIndex, Vec<NodeIndex>> = HashMap::new();

    // Initialize parents map with sorted node indices
    let mut nodes: Vec<_> = graph.node_indices().collect();
    nodes.sort();
    for node in nodes {
        parents.insert(node, vec![]);
    }

    // Collect and sort edges before processing
    let mut edges: Vec<_> = graph.edge_indices().collect();
    edges.sort();

    for edge in edges {
        let (source, target) = graph.edge_endpoints(edge).unwrap();
        let weight = graph.edge_weight(edge).unwrap();

        if let Some(&source_cost) = node_costs.get(&source)
            && let Some(&target_cost) = node_costs.get(&target)
            && target_cost == source_cost + weight
        {
            let parent_vec = parents.get_mut(&target).unwrap();
            parent_vec.push(source);
            parent_vec.sort(); // Sort parents for deterministic order
        }
    }

    let mut all_paths = vec![];
    let mut stack = vec![(end_idx, vec![end_idx])];

    while let Some((node, path)) = stack.pop() {
        if node == start_idx {
            let mut correct_order_path = path.clone();
            correct_order_path.reverse();
            all_paths.push(correct_order_path);
        } else {
            // Get sorted parents for deterministic processing
            let mut sorted_parents: Vec<_> = parents[&node].iter().collect();
            sorted_parents.sort();

            for &parent in sorted_parents {
                let mut new_path = path.clone();
                new_path.push(parent);
                stack.push((parent, new_path));
            }
        }
    }

    // Sort final paths for consistent output
    all_paths.sort();
    all_paths
}

pub fn manhattan_distance(from: (usize, usize), to: (usize, usize)) -> usize {
    let (from_x, from_y) = from;
    let (to_x, to_y) = to;
    ((from_x as isize - to_x as isize).abs() + (from_y as isize - to_y as isize).abs()) as usize
}

pub fn manhattan_distance_i(from: (isize, isize), to: (isize, isize)) -> isize {
    let (from_x, from_y) = from;
    let (to_x, to_y) = to;
    (from_x - to_x).abs() + (from_y - to_y).abs()
}

pub fn digits_to_int(digits: &[isize]) -> isize {
    digits
        .iter()
        .map(|&d| d.to_string())
        .collect::<String>()
        .parse::<isize>()
        .unwrap_or(0)
}

pub fn int_to_digits(int: isize) -> Vec<isize> {
    int.to_string()
        .chars()
        .map(|d| d.to_digit(10).unwrap() as isize)
        .collect::<Vec<_>>()
}

pub fn int_to_instruction(int: isize) -> isize {
    int % 100
}

pub fn int_to_modes(int: isize) -> Vec<isize> {
    let mut digits = int_to_digits(int);
    if digits.len() < 2 {
        return vec![];
    }
    digits.drain(digits.len() - 2..);
    digits.reverse();
    digits
}

#[cfg(test)]
mod tests {
    use petgraph::Graph;

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

    fn build_test_graph() -> (Graph<(), f64>, HashMap<NodeIndex, f64>) {
        let mut graph = Graph::<(), f64>::new();
        let mut costs = HashMap::new();

        // Create a simple graph:
        // A(0) -> B(1) -> C(2)
        //    \-----------> D(3)
        let a = graph.add_node(());
        let b = graph.add_node(());
        let c = graph.add_node(());
        let d = graph.add_node(());

        graph.add_edge(a, b, 1.0);
        graph.add_edge(b, c, 1.0);
        graph.add_edge(a, d, 2.0);

        costs.insert(a, 0.0);
        costs.insert(b, 1.0);
        costs.insert(c, 2.0);
        costs.insert(d, 2.0);

        (graph, costs)
    }

    #[test]
    fn test_single_path() {
        let (graph, costs) = build_test_graph();
        let a = NodeIndex::new(0);
        let b = NodeIndex::new(1);

        let paths = get_all_paths(&graph, &costs, a, b);
        assert_eq!(paths.len(), 1);
        assert_eq!(paths[0], vec![a, b]);
    }

    #[test]
    fn test_multiple_paths() {
        let (graph, costs) = build_test_graph();
        let a = NodeIndex::new(0);
        let d = NodeIndex::new(3);

        let paths = get_all_paths(&graph, &costs, a, d);
        assert_eq!(paths.len(), 1);
        assert_eq!(paths[0], vec![a, d]);
    }

    #[test]
    fn test_no_path() {
        let (graph, costs) = build_test_graph();
        let c = NodeIndex::new(2);
        let a = NodeIndex::new(0);

        let paths = get_all_paths(&graph, &costs, c, a);
        assert_eq!(paths.len(), 0);
    }

    #[test]
    fn test_same_node() {
        let (graph, costs) = build_test_graph();
        let a = NodeIndex::new(0);

        let paths = get_all_paths(&graph, &costs, a, a);
        assert_eq!(paths.len(), 1);
        assert_eq!(paths[0], vec![a]);
    }

    #[test]
    fn test_digits_to_int() {
        let digits = vec![1, 0, 0, 2];
        let result = digits_to_int(&digits);
        assert_eq!(result, 1002);
    }

    #[test]
    fn test_int_to_digits() {
        let number = 1002;
        let result = int_to_digits(number);
        assert_eq!(result, vec![1, 0, 0, 2]);
    }

    #[test]
    fn test_int_to_instruction_one() {
        let number = 1002;
        let result = int_to_instruction(number);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_int_to_instruction_two() {
        let number = 1010101003;
        let result = int_to_instruction(number);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_int_to_modes_one() {
        let number = 1002;
        let result = int_to_modes(number);
        assert_eq!(result, [1, 0].into_iter().rev().collect::<Vec<_>>());
    }

    #[test]
    fn test_int_to_modes_two() {
        let number = 1010101003;
        let result = int_to_modes(number);
        assert_eq!(
            result,
            [1, 0, 1, 0, 1, 0, 1, 0]
                .into_iter()
                .rev()
                .collect::<Vec<_>>()
        );
    }
}
