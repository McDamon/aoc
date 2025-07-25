// https://adventofcode.com/2019/day/3

use std::collections::HashSet;

use crate::utils::{MoveDir, get_lines, manhattan_distance_i};

struct WireMove {
    dir: MoveDir,
    dist: isize,
}

struct Input {
    wire1_path: Vec<WireMove>,
    wire2_path: Vec<WireMove>,
}

fn parse_input(input_file: &str) -> Input {
    let lines = get_lines(input_file);

    let wire1_path: Vec<WireMove> = parse_wire_path(&lines[0]);
    let wire2_path: Vec<WireMove> = parse_wire_path(&lines[1]);

    Input {
        wire1_path,
        wire2_path,
    }
}

fn parse_wire_path(line: &str) -> Vec<WireMove> {
    line.split(',')
        .map(|s| {
            let (dir, dist) = s.split_at(1);
            let dir = match dir {
                "U" => MoveDir::Up,
                "D" => MoveDir::Down,
                "L" => MoveDir::Left,
                "R" => MoveDir::Right,
                _ => panic!("Invalid direction"),
            };
            let dist = dist.parse::<isize>().expect("Invalid distance");
            WireMove { dir, dist }
        })
        .collect()
}

fn get_wire_moves(wire_path: &Vec<WireMove>, start_pos: (isize, isize)) -> Vec<(isize, isize)> {
    let mut wire_moves: Vec<(isize, isize)> = vec![];
    let mut wire_pos = start_pos;

    for wire_move in wire_path {
        match wire_move.dir {
            MoveDir::Up => {
                let start_y = wire_pos.1;
                for i in 1..wire_move.dist + 1 {
                    let new_pos = (wire_pos.0, start_y + i);
                    wire_moves.push(new_pos);
                    wire_pos = new_pos;
                }
            }
            MoveDir::Down => {
                let start_y = wire_pos.1;
                for i in 1..wire_move.dist + 1 {
                    let new_pos = (wire_pos.0, start_y - i);
                    wire_moves.push(new_pos);
                    wire_pos = new_pos;
                }
            }
            MoveDir::Left => {
                let start_x = wire_pos.0;
                for i in 1..wire_move.dist + 1 {
                    let new_pos = (start_x - i, wire_pos.1);
                    wire_moves.push(new_pos);
                    wire_pos = new_pos;
                }
            }
            MoveDir::Right => {
                let start_x = wire_pos.0;
                for i in 1..wire_move.dist + 1 {
                    let new_pos = (start_x + i, wire_pos.1);
                    wire_moves.push(new_pos);
                    wire_pos = new_pos;
                }
            }
        }
    }

    wire_moves
}

pub fn get_closest_dist(input_file: &str) -> isize {
    let input = parse_input(input_file);

    let start_pos = (0isize, 0isize);

    let wire1_moves = get_wire_moves(&input.wire1_path, start_pos);

    let wire2_moves = get_wire_moves(&input.wire2_path, start_pos);

    let wire1_moves_set: HashSet<(isize, isize)> = HashSet::from_iter(wire1_moves);
    let wire2_moves_set: HashSet<(isize, isize)> = HashSet::from_iter(wire2_moves);

    let intersections: Vec<(isize, isize)> = wire1_moves_set
        .intersection(&wire2_moves_set)
        .cloned()
        .collect();

    let mut manhattan_distances = vec![];

    for intersection in intersections {
        manhattan_distances.push(manhattan_distance_i(start_pos, intersection));
    }

    *manhattan_distances.iter().min().unwrap()
}

fn get_combined_steps(wire_moves: &Vec<(isize, isize)>, intersection: (isize, isize)) -> isize {
    let mut steps = 0;
    for wire_move in wire_moves {
        steps += 1;
        if *wire_move == intersection {
            return steps;
        }
    }

    steps
}

pub fn get_closest_combined_steps(input_file: &str) -> isize {
    let input = parse_input(input_file);

    let start_pos = (0isize, 0isize);

    let wire1_moves = get_wire_moves(&input.wire1_path, start_pos);

    let wire2_moves = get_wire_moves(&input.wire2_path, start_pos);

    let wire1_moves_set: HashSet<(isize, isize)> = HashSet::from_iter(wire1_moves.clone());
    let wire2_moves_set: HashSet<(isize, isize)> = HashSet::from_iter(wire2_moves.clone());

    let intersections: Vec<(isize, isize)> = wire1_moves_set
        .intersection(&wire2_moves_set)
        .cloned()
        .collect();

    let mut combined_steps = vec![];
    for intersection in intersections {
        let wire1_combined_steps_for_intersection = get_combined_steps(&wire1_moves, intersection);
        let wire2_combined_steps_for_intersection = get_combined_steps(&wire2_moves, intersection);
        combined_steps
            .push(wire1_combined_steps_for_intersection + wire2_combined_steps_for_intersection);
    }
    *combined_steps.iter().min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_closest_dist_test01() {
        assert_eq!(6, get_closest_dist("input/2019/day03_test01.txt"));
    }

    #[test]
    fn test_get_closest_dist_test02() {
        assert_eq!(159, get_closest_dist("input/2019/day03_test02.txt"));
    }

    #[test]
    fn test_get_closest_dist_test03() {
        assert_eq!(135, get_closest_dist("input/2019/day03_test03.txt"));
    }

    #[test]
    fn test_get_closest_dist() {
        assert_eq!(293, get_closest_dist("input/2019/day03.txt"));
    }

    #[test]
    fn test_get_closest_combined_steps_test01() {
        assert_eq!(
            30,
            get_closest_combined_steps("input/2019/day03_test01.txt")
        );
    }

    #[test]
    fn test_get_closest_combined_steps_test02() {
        assert_eq!(
            610,
            get_closest_combined_steps("input/2019/day03_test02.txt")
        );
    }

    #[test]
    fn test_get_closest_combined_steps_test03() {
        assert_eq!(
            410,
            get_closest_combined_steps("input/2019/day03_test03.txt")
        );
    }

    #[test]
    fn test_get_closest_combined_steps() {
        assert_eq!(27306, get_closest_combined_steps("input/2019/day03.txt"));
    }
}
