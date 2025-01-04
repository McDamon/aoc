// https://adventofcode.com/2024/day/15

use core::panic;
use std::collections::{HashSet, VecDeque};

use crate::utils::{ArenaTree, get_lines};

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, Default)]
enum MoveDir {
    #[default]
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq, Hash, Default, Clone)]
struct Move {
    pos: (usize, usize),
    entry: char,
    dir: MoveDir,
}

#[derive(Debug)]
struct Input {
    warehouse: Vec<Vec<char>>,
    moves: Vec<MoveDir>,
}

fn parse_warehouse(warehouse_part: &[&str]) -> Vec<Vec<char>> {
    warehouse_part
        .iter()
        .map(|line| line.chars().collect())
        .collect()
}

fn parse_moves(moves_part: &[&str]) -> Vec<MoveDir> {
    let mut moves: Vec<MoveDir> = vec![];
    for move_line in moves_part {
        move_line.chars().for_each(|c| {
            match c {
                '^' => moves.push(MoveDir::Up),
                'v' => moves.push(MoveDir::Down),
                '<' => moves.push(MoveDir::Left),
                '>' => moves.push(MoveDir::Right),
                _ => panic!("Unknown move"),
            };
        });
    }
    moves
}

fn parse_input(input_file: &str) -> Input {
    let lines = get_lines(input_file);

    let parts: Vec<_> = lines.split(|line| line.trim().is_empty()).collect();
    let warehouse_part: Vec<&str> = parts
        .first()
        .map(|v| v.iter().map(|s| s.as_str()).collect())
        .unwrap_or(vec![]);
    let moves_part: Vec<&str> = parts
        .get(1)
        .map(|v| v.iter().map(|s| s.as_str()).collect())
        .unwrap_or(vec![]);

    let warehouse = parse_warehouse(&warehouse_part);

    let moves = parse_moves(&moves_part);

    Input { warehouse, moves }
}

fn print_warehouse(warehouse: &[Vec<char>]) {
    for row in warehouse {
        for entry in row {
            print!("{}", entry);
        }
        println!();
    }
}

fn get_next_move(move_pos: (usize, usize), move_dir: &MoveDir) -> (usize, usize) {
    let (move_x, move_y) = move_pos;
    match move_dir {
        MoveDir::Up => (move_x, move_y - 1),
        MoveDir::Down => (move_x, move_y + 1),
        MoveDir::Left => (move_x - 1, move_y),
        MoveDir::Right => (move_x + 1, move_y),
    }
}

fn perform_move(warehouse: &mut [Vec<char>], robot_pos: &mut (usize, usize), move_dir: &MoveDir) {
    //println!("MoveDir: {:?}", move_dir);
    let (robot_x, robot_y) = robot_pos;
    let mut maybe_next_move = Some(get_next_move((*robot_x, *robot_y), move_dir));
    while let Some(next_move) = maybe_next_move {
        let (next_x, next_y) = next_move;
        match warehouse[next_y][next_x] {
            '.' => {
                warehouse[*robot_y][*robot_x] = '.';
                warehouse[next_y][next_x] = '@';
                *robot_x = next_x;
                *robot_y = next_y;
                maybe_next_move = None;
            }
            'O' => {
                let (peek_x, peek_y) = get_next_move((next_x, next_y), move_dir);
                match warehouse[peek_y][peek_x] {
                    '.' => {
                        warehouse[next_y][next_x] = '.';
                        warehouse[peek_y][peek_x] = 'O';
                        maybe_next_move = Some(get_next_move((*robot_x, *robot_y), move_dir))
                    }
                    'O' => maybe_next_move = Some((peek_x, peek_y)),
                    _ => maybe_next_move = None,
                }
            }
            _ => {
                maybe_next_move = None;
            }
        }
    }

    //print_warehouse(warehouse);
    //println!();
}

fn get_robot_pos(warehouse: &[Vec<char>]) -> (usize, usize) {
    for (y, row) in warehouse.iter().enumerate() {
        for (x, entry) in row.iter().enumerate() {
            if *entry == '@' {
                return (x, y);
            }
        }
    }
    panic!("Robot not found in the warehouse");
}

fn get_sum_gps(input_file: &str) -> u32 {
    let input = parse_input(input_file);

    let mut warehouse = input.warehouse.clone();

    let mut robot_pos: (usize, usize) = get_robot_pos(warehouse.as_slice());

    /*println!("Initial robot pos: {:?}", robot_pos);
    println!("Initial state:");
    print_warehouse(&warehouse);
    println!();*/

    for move_dir in &input.moves {
        perform_move(&mut warehouse, &mut robot_pos, move_dir);
    }

    warehouse.iter().enumerate().fold(0, |acc, (i, row)| {
        acc + row
            .iter()
            .enumerate()
            .fold(0, |acc, (j, entry)| match entry {
                'O' => acc + (100 * i as u32 + j as u32),
                _ => acc,
            })
    })
}

//
// Part 2
//

fn print_tree(tree: &ArenaTree<Move>) {
    for node in tree.arena.iter() {
        println!("Node: {:?}", node);
    }
}

fn widen_warehouse(warehouse: &[Vec<char>]) -> Vec<Vec<char>> {
    warehouse.iter().fold(vec![], |mut acc, row| {
        let mut new_row: Vec<char> = vec![];
        for entry in row {
            match entry {
                '.' => {
                    new_row.push('.');
                    new_row.push('.');
                }
                '#' => {
                    new_row.push('#');
                    new_row.push('#');
                }
                'O' => {
                    new_row.push('[');
                    new_row.push(']');
                }
                '@' => {
                    new_row.push('@');
                    new_row.push('.');
                }
                _ => panic!("Unknown entry"),
            }
        }
        acc.push(new_row);
        acc
    })
}

fn get_pot_move(move_pos: (usize, usize), move_dir: MoveDir) -> (usize, usize) {
    let (move_x, move_y) = move_pos;
    match move_dir {
        MoveDir::Left => (move_x - 1, move_y),
        MoveDir::Right => (move_x + 1, move_y),
        MoveDir::Up => (move_x, move_y - 1),
        MoveDir::Down => (move_x, move_y + 1),
    }
}

fn is_valid_pot_move(
    warehouse: &[Vec<char>],
    visited_pos: &mut HashSet<(usize, usize)>,
    move_pos: &(usize, usize),
) -> bool {
    let (pos_x, pos_y) = move_pos;
    if *pos_x >= warehouse[0].len() || *pos_y >= warehouse.len() {
        println!(
            "Move to position {:?} is not valid: out of bounds",
            move_pos
        );
        return false;
    }
    if visited_pos.contains(move_pos) {
        println!(
            "Move to position {:?} is not valid: already visited",
            move_pos
        );
        return false;
    }
    let entry = warehouse[*pos_y][*pos_x];
    let is_valid = matches!(entry, '@' | '[' | ']');
    if !is_valid {
        println!(
            "Move to position {:?} is not valid: invalid entry '{}'",
            move_pos, entry
        );
    } else {
        println!("Move to position {:?} is valid", move_pos);
    }
    is_valid
}

fn build_moves_wider_dfs(
    warehouse: &[Vec<char>],
    visited_pos: &mut HashSet<(usize, usize)>,
    stored_moves: &mut Vec<Move>,
    move_pos: (usize, usize),
    move_dir: MoveDir,
) {
    let mut stack = vec![move_pos];

    while !stack.is_empty() {
        if let Some(move_pos) = stack.pop() {
            if !is_valid_pot_move(warehouse, visited_pos, &move_pos) {
                continue;
            }

            visited_pos.insert(move_pos);

            let (move_x, move_y) = move_pos;
            let entry = warehouse[move_y][move_x];

            println!("Storing {:?} move {:?}", move_dir, move_pos);
            let next_move = Move {
                pos: move_pos,
                entry,
                dir: move_dir,
            };
            stored_moves.push(next_move);

            match entry {
                '[' if warehouse[move_y][move_x + 1] == ']'
                    && (move_dir == MoveDir::Up || move_dir == MoveDir::Down) =>
                {
                    let adj_move_pos = (move_x + 1, move_y);
                    stack.push(adj_move_pos);
                }
                ']' if warehouse[move_y][move_x - 1] == '['
                    && (move_dir == MoveDir::Up || move_dir == MoveDir::Down) =>
                {
                    let adj_move_pos = (move_x - 1, move_y);
                    stack.push(adj_move_pos);
                }
                _ => (),
            }

            let pot_move_pos = get_pot_move(move_pos, move_dir);
            stack.push(pot_move_pos);
        }
    }
}

fn build_moves_wider_bfs(
    warehouse: &[Vec<char>],
    visited_pos: &mut HashSet<(usize, usize)>,
    stored_moves: &mut Vec<Move>,
    move_pos: (usize, usize),
    move_dir: MoveDir,
) {
    let mut queue = VecDeque::new();
    queue.push_back(move_pos);
    visited_pos.insert(move_pos);

    while !queue.is_empty() {
        if let Some(move_pos) = queue.pop_front() {
            let (move_x, move_y) = move_pos;
            let entry = warehouse[move_y][move_x];

            println!("Storing {:?} move {:?}", move_dir, move_pos);
            let next_move = Move {
                pos: move_pos,
                entry,
                dir: move_dir,
            };
            stored_moves.push(next_move);

            match entry {
                '[' if warehouse[move_y][move_x + 1] == ']'
                    && (move_dir == MoveDir::Up || move_dir == MoveDir::Down) =>
                {
                    let adj_move_pos = (move_x + 1, move_y);
                    if is_valid_pot_move(warehouse, visited_pos, &adj_move_pos) {
                        visited_pos.insert(adj_move_pos);
                        queue.push_back(adj_move_pos);
                    } else {
                        if let Some(wall_pos) = found_wall(warehouse, stored_moves, adj_move_pos) {
                            println!("Found wall at {:?}", wall_pos);
                            break;
                        }
                    }
                }
                ']' if warehouse[move_y][move_x - 1] == '['
                    && (move_dir == MoveDir::Up || move_dir == MoveDir::Down) =>
                {
                    let adj_move_pos = (move_x - 1, move_y);
                    if is_valid_pot_move(warehouse, visited_pos, &adj_move_pos) {
                        visited_pos.insert(adj_move_pos);
                        queue.push_back(adj_move_pos);
                    } else {
                        if let Some(wall_pos) = found_wall(warehouse, stored_moves, adj_move_pos) {
                            println!("Found wall at {:?}", wall_pos);
                            break;
                        }
                    }
                }
                _ => (),
            }

            let pot_move_pos = get_pot_move(move_pos, move_dir);
            if is_valid_pot_move(warehouse, visited_pos, &pot_move_pos) {
                visited_pos.insert(pot_move_pos);
                queue.push_back(pot_move_pos);
            } else {
                if let Some(wall_pos) = found_wall(warehouse, stored_moves, pot_move_pos) {
                    println!("Found wall at {:?}", wall_pos);
                    break;
                }
            }
        }
    }
}

fn found_wall(
    warehouse: &[Vec<char>],
    stored_moves: &mut Vec<Move>,
    move_pos: (usize, usize),
) -> Option<(usize, usize)> {
    let (move_x, move_y) = move_pos;
    let entry = warehouse[move_y][move_x];
    if entry == '#' {
        println!("Found wall at {:?}", move_pos);
        stored_moves.clear();
        return Some(move_pos);
    }
    None
}

fn make_moves(warehouse: &mut [Vec<char>], stored_moves: &Vec<Move>) {
    for m in stored_moves.iter().rev() {
        let (from_x, from_y) = m.pos;
        let (to_x, to_y) = get_pot_move(m.pos, m.dir);

        let from_entry = warehouse[from_y][from_x];
        let to_entry = warehouse[to_y][to_x];

        println!(
            "Moving {:?} from {:?} to {:?}, replacing {:?}",
            from_entry,
            (from_x, from_y),
            (to_x, to_y),
            to_entry
        );

        warehouse[from_y][from_x] = to_entry;
        warehouse[to_y][to_x] = from_entry;
    }
}

fn get_sum_gps_wider(input_file: &str) -> u32 {
    let input = parse_input(input_file);

    let mut warehouse_wider = widen_warehouse(&input.warehouse);

    println!("Initial state:");
    print_warehouse(&warehouse_wider);
    println!();

    for move_dir in input.moves {
        let mut visited_pos: HashSet<(usize, usize)> = HashSet::new();
        let mut stored_moves = vec![];
        let robot_pos: (usize, usize) = get_robot_pos(&warehouse_wider);
        build_moves_wider_bfs(
            &warehouse_wider,
            &mut visited_pos,
            &mut stored_moves,
            robot_pos,
            move_dir,
        );
        make_moves(&mut warehouse_wider, &stored_moves);
        print_warehouse(&warehouse_wider);
        println!();
    }

    warehouse_wider.iter().enumerate().fold(0, |acc, (y, row)| {
        acc + row
            .iter()
            .enumerate()
            .fold(0, |acc, (x, entry)| match entry {
                '[' => {
                    let rhs_box_half = row[x + 1];
                    if rhs_box_half == ']' {
                        acc + (100 * y as u32 + x as u32)
                    } else {
                        acc
                    }
                }
                ']' => {
                    let lhs_box_half = row[x - 1];
                    if lhs_box_half != '[' {
                        panic!("Invalid box half");
                    } else {
                        acc
                    }
                }
                _ => acc,
            })
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_sum_gps_test01() {
        assert_eq!(10092, get_sum_gps("input/2024/day15_test01.txt"));
    }

    #[test]
    fn test_get_sum_gps_test02() {
        assert_eq!(2028, get_sum_gps("input/2024/day15_test02.txt"));
    }

    #[test]
    fn test_get_sum_gps_test03() {
        // Down
        assert_eq!(1624, get_sum_gps("input/2024/day15_test03.txt"));
    }

    #[test]
    fn test_get_sum_gps_test04() {
        // Right
        assert_eq!(1626, get_sum_gps("input/2024/day15_test04.txt"));
    }

    #[test]
    fn test_get_sum_gps_test05() {
        // Down
        assert_eq!(2024, get_sum_gps("input/2024/day15_test05.txt"));
    }

    #[test]
    fn test_get_sum_gps_test06() {
        // Left
        assert_eq!(1621, get_sum_gps("input/2024/day15_test06.txt"));
    }

    #[test]
    fn test_get_sum_gps_test07() {
        // Up
        assert_eq!(1224, get_sum_gps("input/2024/day15_test07.txt"));
    }

    #[test]
    fn test_get_sum_gps_test08() {
        // Right
        assert_eq!(1627, get_sum_gps("input/2024/day15_test08.txt"));
    }

    #[test]
    fn test_get_sum_gps() {
        assert_eq!(1517819, get_sum_gps("input/2024/day15.txt"));
    }

    //
    // Wider
    //

    #[test]
    fn test_get_sum_gps_wider_test01() {
        // From Part 1
        assert_eq!(9021, get_sum_gps_wider("input/2024/day15_test01.txt"));
    }

    #[test]
    fn test_get_sum_gps_wider_test02() {
        // Also from Part 1
        assert_eq!(1751, get_sum_gps_wider("input/2024/day15_test02.txt"));
    }

    #[test]
    fn test_get_sum_gps_wider_test03() {
        // Down, no boxes, blocked by wall
        assert_eq!(1648, get_sum_gps_wider("input/2024/day15_test03.txt"));
    }

    #[test]
    fn test_get_sum_gps_wider_test04() {
        // Right
        assert_eq!(1652, get_sum_gps_wider("input/2024/day15_test04.txt"));
    }

    #[test]
    fn test_get_sum_gps_wider_test05() {
        // Down
        assert_eq!(2048, get_sum_gps_wider("input/2024/day15_test05.txt"));
    }

    #[test]
    fn test_get_sum_gps_wider_test06() {
        // Left
        assert_eq!(1642, get_sum_gps_wider("input/2024/day15_test06.txt"));
    }

    #[test]
    fn test_get_sum_gps_wider_test07() {
        // Up
        assert_eq!(1248, get_sum_gps_wider("input/2024/day15_test07.txt"));
    }

    #[test]
    fn test_get_sum_gps_wider_test08() {
        // Right
        assert_eq!(1654, get_sum_gps_wider("input/2024/day15_test08.txt"));
    }

    #[test]
    fn test_get_sum_gps_wider_test09() {
        // Part 2 example
        assert_eq!(618, get_sum_gps_wider("input/2024/day15_test09.txt"));
    }

    #[test]
    fn test_get_sum_gps_wider_test10() {
        // Test all directions
        assert_eq!(9235, get_sum_gps_wider("input/2024/day15_test10.txt"));
    }

    #[test]
    fn test_get_sum_gps_wider_test11() {
        // Left
        assert_eq!(406, get_sum_gps_wider("input/2024/day15_test11.txt"));
    }

    #[test]
    fn test_get_sum_gps_wider_test12() {
        // Up, blocked by wall
        assert_eq!(509, get_sum_gps_wider("input/2024/day15_test12.txt"));
    }

    #[test]
    fn test_get_sum_gps_wider_test13() {
        // Up, blocked by wall (alt)
        assert_eq!(511, get_sum_gps_wider("input/2024/day15_test13.txt"));
    }

    #[test]
    fn test_get_sum_gps_wider_test14() {
        // Around and then up
        assert_eq!(816, get_sum_gps_wider("input/2024/day15_test14.txt"));
    }

    #[test]
    fn test_get_sum_gps_wider_test15() {
        // Around and then up (alt)
        assert_eq!(822, get_sum_gps_wider("input/2024/day15_test15.txt"));
    }

    #[test]
    fn test_get_sum_gps_wider_test16() {
        // Checks corners with wall
        assert_eq!(1430, get_sum_gps_wider("input/2024/day15_test16.txt"));
    }

    #[test]
    fn test_get_sum_gps_wider_test17() {
        // Checks corners with wall (alt)
        assert_eq!(1216, get_sum_gps_wider("input/2024/day15_test17.txt"));
    }

    #[test]
    fn test_get_sum_gps_wider() {
        // Our test input
        assert_eq!(0, get_sum_gps_wider("input/2024/day15.txt"));
    }
}
