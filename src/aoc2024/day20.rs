// https://adventofcode.com/2024/day/20

use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

use petgraph::{
    algo,
    graph::{DiGraph, NodeIndex},
    Graph,
};

use crate::utils::{get_lines, Direction};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Move {
    pos: (usize, usize),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Cheat {
    from: (usize, usize),
    to: (usize, usize),
    distance: usize,
}

#[derive(Debug)]
struct Input {
    track: HashMap<(usize, usize), char>,
}

fn parse_input(input_file: &str) -> Input {
    let lines = get_lines(input_file);

    let mut track: HashMap<(usize, usize), char> = HashMap::new();

    for (y, line) in lines.into_iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            track.insert((x, y), c);
        }
    }

    Input { track }
}

fn print_track(track: &HashMap<(usize, usize), char>) {
    let max_x = track.keys().map(|(x, _)| *x).max().unwrap();
    let max_y = track.keys().map(|(_, y)| *y).max().unwrap();

    for y in 0..=max_y {
        for x in 0..=max_x {
            if let Some(c) = track.get(&(x, y)) {
                print!("{}", c);
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

fn build_graph(
    track: &HashMap<(usize, usize), char>,
    maybe_cheat: Option<Cheat>,
) -> (Graph<Move, f64>, HashMap<Move, NodeIndex>) {
    // Create directed graph
    let mut graph = DiGraph::<Move, f64>::new();
    let mut node_indices: HashMap<Move, NodeIndex> = HashMap::new();

    // Add nodes and edges
    // First create all nodes
    for (&(x, y), &c) in track.iter() {
        if c != '#' {
            let point = Move {
                pos: (x, y),
            };
            let node_idx = graph.add_node(point);
            node_indices.insert(point, node_idx);
            //println!("Added node {:?}", graph[node_idx]);
        }
    }

    // Then a~dd all edges in a separate pass
    for (&(x, y), &c) in track.iter() {
        if c != '#' {
            for dir in Direction::all() {
                let point = Move {
                    pos: (x, y),
                };
                let node_idx = node_indices[&point];

                // Forward movement
                let (dx, dy) = dir.to_delta();
                let (next_x, next_y) = (x as isize + dx, y as isize + dy);
                if let Some(&next_c) = track.get(&(next_x as usize, next_y as usize)) {
                    if next_c != '#' {
                        let next_move = Move {
                            pos: (next_x as usize, next_y as usize),
                        };
                        let next_idx = node_indices[&next_move];
                        graph.add_edge(node_idx, next_idx, 1.0);
                        //println!("Added edge {:?} -> {:?}", graph[node_idx], graph[next_idx]);
                    }
                }
            }
        }
    }

    if let Some(cheat) = maybe_cheat {
        let from = Move { pos: cheat.from };
        let to = Move { pos: cheat.to };
        if let Some(from_idx) = node_indices.get(&from)
            && let Some(to_idx) = node_indices.get(&to)
        {
            graph.add_edge(*from_idx, *to_idx, cheat.distance as f64);
            /*println!(
                "Added cheat edge {:?} -> {:?}",
                graph[*from_idx], graph[*to_idx]
            );*/
        }
    }

    (graph, node_indices)
}

fn manhattan_distance(from: (usize, usize), to: (usize, usize)) -> usize {
    let (from_x, from_y) = from;
    let (to_x, to_y) = to;
    ((from_x as isize - to_x as isize).abs() + (from_y as isize - to_y as isize).abs()) as usize
}

fn get_cheats(track: &HashMap<(usize, usize), char>, max_distance: usize) -> HashSet<Cheat> {
    let mut cheats = HashSet::new();
    for ((track_x, track_y), track_entry) in track.iter() {
        for ((next_track_x, next_track_y), next_track_entry) in track.iter() {
            if (track_x, track_y) != (next_track_x, next_track_y) && *track_entry != '#' && *next_track_entry != '#' {
                let distance =
                    manhattan_distance((*track_x, *track_y), (*next_track_x, *next_track_y));
                if distance <= max_distance {
                    /*println!(
                        "Adding cheat from {:?}:{:?} to {:?}:{:?} with distance {}",
                        (*track_x, *track_y),
                        track_entry,
                        (*next_track_x, *next_track_y),
                        next_track_entry,
                        distance
                    );*/
                    cheats.insert(Cheat {
                        from: (*track_x, *track_y),
                        to: (*next_track_x, *next_track_y),
                        distance,
                    });
                }
            }
        }
    }
    cheats
}

fn get_distance(
    start: (usize, usize),
    end: (usize, usize),
    track: &HashMap<(usize, usize), char>,
    maybe_cheat: Option<Cheat>,
) -> Option<usize> {
    let (graph, node_indices) = build_graph(track, maybe_cheat);

    let start_move = Move { pos: start };
    let end_move = Move { pos: end };

    let start_idx = node_indices[&start_move];
    let end_idx = node_indices[&end_move];

    if let Some((distance, _path)) = algo::astar(
        &graph,
        start_idx,
        |finish| finish == end_idx,
        |e| *e.weight() as usize,
        |_| 0,
    ) {
        Some(distance)
    } else {
        None
    }
}

fn get_cheat_saving(
    start: (usize, usize),
    end: (usize, usize),
    track: &HashMap<(usize, usize), char>,
    cheat: Cheat,
    no_cheat_distance: usize,
) -> Option<usize> {
    if let Some(distance) = get_distance(start, end, track, Some(cheat)) {
        let cheat_saving = no_cheat_distance - distance;
        Some(cheat_saving)
    } else {
        None
    }
}

fn get_num_cheats(
    input_file: &str,
    required_savings: usize,
    is_at_least: bool,
    max_distance: usize,
) -> usize {
    let input = parse_input(input_file);

    //print_track(&input.track);

    // Find start and end points
    let start = input
        .track
        .iter()
        .find(|(_, c)| **c == 'S')
        .map(|(&pos, _)| pos)
        .unwrap();
    let end = input
        .track
        .iter()
        .find(|(_, c)| **c == 'E')
        .map(|(&pos, _)| pos)
        .unwrap();

    let cheats: HashSet<Cheat> = get_cheats(&input.track, max_distance);

    let no_cheat_distance = if let Some(distance) = get_distance(start, end, &input.track, None) {
        distance
    } else {
        return 0;
    };

    let mut cheat_savings: HashMap<Cheat, usize> = HashMap::new();

    for cheat in cheats {
        if let Some(cheat_saving) =
            get_cheat_saving(start, end, &input.track, cheat, no_cheat_distance)
        {
            cheat_savings.insert(cheat, cheat_saving);
        }
    }

    let filtered_cheat_savings = cheat_savings
        .iter()
        .filter(|&(_c, s)| {
            if is_at_least {
                *s >= required_savings
            } else {
                *s == required_savings
            }
        })
        .collect::<Vec<_>>();

    filtered_cheat_savings.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_num_cheats_test01() {
        assert_eq!(
            14,
            get_num_cheats("input/2024/day20_test01.txt", 2, false, 2)
        );
    }

    #[test]
    fn test_get_num_cheats_test02() {
        assert_eq!(
            14,
            get_num_cheats("input/2024/day20_test01.txt", 4, false, 2)
        );
    }

    #[test]
    fn test_get_num_cheats_test03() {
        assert_eq!(
            2,
            get_num_cheats("input/2024/day20_test01.txt", 6, false, 2)
        );
    }

    #[test]
    fn test_get_num_cheats_test04() {
        assert_eq!(
            4,
            get_num_cheats("input/2024/day20_test01.txt", 8, false, 2)
        );
    }

    #[test]
    fn test_get_num_cheats_test05() {
        assert_eq!(
            2,
            get_num_cheats("input/2024/day20_test01.txt", 10, false, 2)
        );
    }

    #[test]
    fn test_get_num_cheats_test06() {
        assert_eq!(
            3,
            get_num_cheats("input/2024/day20_test01.txt", 12, false, 2)
        );
    }

    #[test]
    fn test_get_num_cheats_test07() {
        assert_eq!(
            1,
            get_num_cheats("input/2024/day20_test01.txt", 20, false, 2)
        );
    }

    #[test]
    fn test_get_num_cheats_test08() {
        assert_eq!(
            1,
            get_num_cheats("input/2024/day20_test01.txt", 36, false, 2)
        );
    }

    #[test]
    fn test_get_num_cheats_test09() {
        assert_eq!(
            1,
            get_num_cheats("input/2024/day20_test01.txt", 38, false, 2)
        );
    }

    #[test]
    fn test_get_num_cheats_test10() {
        assert_eq!(
            1,
            get_num_cheats("input/2024/day20_test01.txt", 40, false, 2)
        );
    }

    #[test]
    fn test_get_num_cheats_test11() {
        assert_eq!(
            1,
            get_num_cheats("input/2024/day20_test01.txt", 64, false, 2)
        );
    }

    #[test]
    fn test_get_num_cheats_test12() {
        assert_eq!(2, get_num_cheats("input/2024/day20_test02.txt", 2, true, 2));
    }

    #[ignore]
    #[test]
    fn test_get_num_cheats() {
        assert_eq!(1399, get_num_cheats("input/2024/day20.txt", 100, true, 2));
    }

    #[test]
    fn test_get_num_cheats_new_rules_test01() {
        assert_eq!(
            32,
            get_num_cheats("input/2024/day20_test01.txt", 50, false, 20)
        );
    }

    #[test]
    fn test_get_num_cheats_new_rules_test02() {
        assert_eq!(
            31,
            get_num_cheats("input/2024/day20_test01.txt", 52, false, 20)
        );
    }

    #[test]
    fn test_get_num_cheats_new_rules_test03() {
        assert_eq!(
            29,
            get_num_cheats("input/2024/day20_test01.txt", 54, false, 20)
        );
    }

    #[test]
    fn test_get_num_cheats_new_rules_test04() {
        assert_eq!(
            39,
            get_num_cheats("input/2024/day20_test01.txt", 56, false, 20)
        );
    }

    #[test]
    fn test_get_num_cheats_new_rules_test05() {
        assert_eq!(
            25,
            get_num_cheats("input/2024/day20_test01.txt", 58, false, 20)
        );
    }

    #[test]
    fn test_get_num_cheats_new_rules_test06() {
        assert_eq!(
            23,
            get_num_cheats("input/2024/day20_test01.txt", 60, false, 20)
        );
    }

    #[test]
    fn test_get_num_cheats_new_rules_test07() {
        assert_eq!(
            20,
            get_num_cheats("input/2024/day20_test01.txt", 62, false, 20)
        );
    }

    #[test]
    fn test_get_num_cheats_new_rules_test08() {
        assert_eq!(
            19,
            get_num_cheats("input/2024/day20_test01.txt", 64, false, 20)
        );
    }

    #[test]
    fn test_get_num_cheats_new_rules_test09() {
        assert_eq!(
            12,
            get_num_cheats("input/2024/day20_test01.txt", 66, false, 20)
        );
    }

    #[test]
    fn test_get_num_cheats_new_rules_test10() {
        assert_eq!(
            14,
            get_num_cheats("input/2024/day20_test01.txt", 68, false, 20)
        );
    }

    #[test]
    fn test_get_num_cheats_new_rules_test11() {
        assert_eq!(
            12,
            get_num_cheats("input/2024/day20_test01.txt", 70, false, 20)
        );
    }

    #[test]
    fn test_get_num_cheats_new_rules_test12() {
        assert_eq!(
            22,
            get_num_cheats("input/2024/day20_test01.txt", 72, false, 20)
        );
    }

    #[test]
    fn test_get_num_cheats_new_rules_test13() {
        assert_eq!(
            4,
            get_num_cheats("input/2024/day20_test01.txt", 74, false, 20)
        );
    }

    #[test]
    fn test_get_num_cheats_new_rules_test14() {
        assert_eq!(
            3,
            get_num_cheats("input/2024/day20_test01.txt", 76, false, 20)
        );
    }

    #[ignore]
    #[test]
    fn test_get_num_cheats_new_rules() {
        assert_eq!(0, get_num_cheats("input/2024/day20.txt", 100, true, 20));
    }
}
