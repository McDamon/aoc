// https://adventofcode.com/2024/day/10

use std::collections::HashSet;

use super::utils::get_lines;
use super::utils::ArenaTree;

struct Input {
    top_map: Vec<Vec<u32>>,
}

fn parse_input(input_file: &str) -> Input {
    let lines = get_lines(input_file);

    let mut top_map: Vec<Vec<u32>> = Vec::new();

    for line in lines {
        let report: Vec<u32> = line
            .chars()
            .map(|level| level.to_string().parse::<u32>().unwrap())
            .collect();
        top_map.push(report);
    }

    Input { top_map }
}

fn print_top_map(top_map: &Vec<Vec<u32>>) {
    for row in top_map {
        for level in row {
            print!("{:?}", level);
        }
        println!();
    }
}

fn print_tree(tree: &ArenaTree<u32>) {
    for node in tree.arena.iter() {
        println!("Node: {:?}", node);
    }
}

fn build_tree(
    tree: &mut ArenaTree<u32>,
    top_map: &Vec<Vec<u32>>,
    curr_level: u32,
    curr_pos: (usize, usize),
    visited_hiking_trails: &mut HashSet<(usize, usize)>,
) {
    if curr_level == 9 && !visited_hiking_trails.contains(&curr_pos) {
        //println!("Found hiking trail at {:?}, level: {:?}, visited", curr_pos, curr_level);
        visited_hiking_trails.insert(curr_pos);
    }

    let (curr_row, curr_col) = curr_pos;
    let n_dir = if curr_row as i32 - 1 < 0 {
        None
    } else {
        Some((curr_row - 1, curr_col))
    };
    let s_dir = if curr_row as i32 + 1 >= top_map.len() as i32 {
        None
    } else {
        Some((curr_row + 1, curr_col))
    };
    let e_dir = if curr_col as i32 + 1 >= top_map[0].len() as i32 {
        None
    } else {
        Some((curr_row, curr_col + 1))
    };
    let w_dir = if curr_col as i32 - 1 < 0 {
        None
    } else {
        Some((curr_row, curr_col - 1))
    };

    let curr_node = tree.add_node(curr_level);

    if let Some((n_row, n_col)) = n_dir
        && let Some(n_level) = top_map.get(n_row).and_then(|row| row.get(n_col))
        && *n_level == curr_level + 1
    {
        /*println!(
            "At {:?}, level: {:?}, moving N, level: {:?}",
            curr_pos, curr_level, n_level
        );*/
        let n_node: usize = tree.add_node(*n_level);
        tree.arena[curr_node].children.push(n_node);
        tree.arena[n_node].parent = Some(curr_node);
        build_tree(
            tree,
            top_map,
            *n_level,
            (n_row, n_col),
            visited_hiking_trails,
        );
    }

    if let Some((s_row, s_col)) = s_dir
        && let Some(s_level) = top_map.get(s_row).and_then(|row| row.get(s_col))
        && *s_level == curr_level + 1
    {
        /*println!(
            "At {:?}, level: {:?}, moving S, level: {:?}",
            curr_pos, curr_level, s_level
        );*/
        let s_node = tree.add_node(*s_level);
        tree.arena[curr_node].children.push(s_node);
        tree.arena[s_node].parent = Some(curr_node);
        build_tree(
            tree,
            top_map,
            *s_level,
            (s_row, s_col),
            visited_hiking_trails,
        );
    }

    if let Some((e_row, e_col)) = e_dir
        && let Some(e_level) = top_map.get(e_row).and_then(|row| row.get(e_col))
        && *e_level == curr_level + 1
    {
        /*println!(
            "At {:?}, level: {:?}, moving E, level: {:?}",
            curr_pos, curr_level, e_level
        );*/
        let e_node = tree.add_node(*e_level);
        tree.arena[curr_node].children.push(e_node);
        tree.arena[e_node].parent = Some(curr_node);
        build_tree(
            tree,
            top_map,
            *e_level,
            (e_row, e_col),
            visited_hiking_trails,
        );
    }

    if let Some((w_row, w_col)) = w_dir
        && let Some(w_level) = top_map.get(w_row).and_then(|row| row.get(w_col))
        && *w_level == curr_level + 1
    {
        /*println!(
            "At {:?}, level: {:?}, moving W, level: {:?}",
            curr_pos, curr_level, w_level
        );*/
        let w_node = tree.add_node(*w_level);
        tree.arena[curr_node].children.push(w_node);
        tree.arena[w_node].parent = Some(curr_node);
        build_tree(
            tree,
            top_map,
            *w_level,
            (w_row, w_col),
            visited_hiking_trails,
        );
    }
}

fn get_sum_trailheads(input_file: &str) -> usize {
    let input = parse_input(input_file);

    let trailheads: Vec<(usize, usize)> = input
        .top_map
        .iter()
        .enumerate()
        .flat_map(|(row_index, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(col_index, level)| {
                    if *level == 0 {
                        Some((row_index, col_index))
                    } else {
                        None
                    }
                })
        })
        .collect::<Vec<(usize, usize)>>();

    let mut sum_trailheads = 0usize;

    for (start_row, start_col) in trailheads {
        let mut visited_hiking_trails: HashSet<(usize, usize)> = HashSet::new();

        let mut tree: ArenaTree<u32> = ArenaTree::default();
        build_tree(
            &mut tree,
            &input.top_map,
            0,
            (start_row, start_col),
            &mut visited_hiking_trails,
        );

        sum_trailheads += visited_hiking_trails.len();

        print_tree(&tree);
    }

    sum_trailheads
}

fn get_sum_trailhead_ratings(input_file: &str) -> usize {
    let input = parse_input(input_file);

    let trailheads: Vec<(usize, usize)> = input
        .top_map
        .iter()
        .enumerate()
        .flat_map(|(row_index, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(col_index, level)| {
                    if *level == 0 {
                        Some((row_index, col_index))
                    } else {
                        None
                    }
                })
        })
        .collect::<Vec<(usize, usize)>>();

    let mut sum_trailhead_ratings = 0usize;

    for (start_row, start_col) in trailheads {
        let mut visited_hiking_trails: HashSet<(usize, usize)> = HashSet::new();

        let mut tree: ArenaTree<u32> = ArenaTree::default();
        build_tree(
            &mut tree,
            &input.top_map,
            0,
            (start_row, start_col),
            &mut visited_hiking_trails,
        );

        sum_trailhead_ratings += tree.arena[tree.depth(8)].children.len();

        //print_tree(&tree);
    }

    sum_trailhead_ratings
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_sum_trailheads_test01() {
        assert_eq!(1, get_sum_trailheads("input/day10_test01.txt"));
    }

    #[test]
    fn test_get_sum_trailheads_test02() {
        assert_eq!(2, get_sum_trailheads("input/day10_test02.txt"));
    }

    #[test]
    fn test_get_sum_trailheads_test03() {
        assert_eq!(4, get_sum_trailheads("input/day10_test03.txt"));
    }

    #[test]
    fn test_get_sum_trailheads_test04() {
        assert_eq!(3, get_sum_trailheads("input/day10_test04.txt"));
    }

    #[test]
    fn test_get_sum_trailheads_test05() {
        assert_eq!(36, get_sum_trailheads("input/day10_test05.txt"));
    }

    #[test]
    fn test_get_sum_trailheads() {
        assert_eq!(782, get_sum_trailheads("input/day10.txt"));
    }

    #[test]
    fn test_get_sum_trailhead_ratings_test01() {
        assert_eq!(16, get_sum_trailhead_ratings("input/day10_test01.txt"));
    }

    #[test]
    fn test_get_sum_trailheads_ratings_test02() {
        assert_eq!(2, get_sum_trailhead_ratings("input/day10_test02.txt"));
    }

    #[test]
    fn test_get_sum_trailheads_ratings_test03() {
        assert_eq!(13, get_sum_trailhead_ratings("input/day10_test03.txt"));
    }

    #[test]
    fn test_get_sum_trailheads_ratings_test04() {
        assert_eq!(3, get_sum_trailhead_ratings("input/day10_test04.txt"));
    }

    #[test]
    fn test_get_sum_trailhead_ratings_test05() {
        assert_eq!(81, get_sum_trailhead_ratings("input/day10_test05.txt"));
    }

    #[test]
    fn test_get_sum_trailhead_ratings_test06() {
        assert_eq!(3, get_sum_trailhead_ratings("input/day10_test06.txt"));
    }

    #[test]
    fn test_get_sum_trailhead_ratings_test07() {
        assert_eq!(227, get_sum_trailhead_ratings("input/day10_test07.txt"));
    }

    #[test]
    fn test_get_sum_trailhead_ratings() {
        assert_eq!(1694, get_sum_trailhead_ratings("input/day10.txt"));
    }
}
