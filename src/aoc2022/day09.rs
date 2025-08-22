// https://adventofcode.com/2022/day/9

use std::collections::HashSet;

use regex::Regex;

use crate::utils::{get_lines, MoveDir};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum MoveCondition {
    Adjacent,
    Lateral,
    Diagonal,
}

#[derive(Debug, PartialEq)]
pub struct Move {
    dir: MoveDir,
    steps: u32,
}

pub fn parse_input(input_file: &str) -> Vec<Move> {
    lazy_static! {
        static ref RE_MOVE: Regex = Regex::new(r"(?P<dir>[A-Z]) (?P<steps>[0-9]+)").unwrap();
    }

    let mut moves: Vec<Move> = vec![];

    let lines = get_lines(input_file);

    for line in lines {
        let caps_move = RE_MOVE.captures(&line);

        if let Some(caps_move) = caps_move {
            let dir_char = caps_move["dir"]
                .chars()
                .next()
                .expect("the string is empty");

            let dir = match dir_char {
                'L' => Some(MoveDir::Left),
                'R' => Some(MoveDir::Right),
                'U' => Some(MoveDir::Up),
                'D' => Some(MoveDir::Down),
                _ => None,
            };

            let steps = caps_move["steps"].to_string().parse().unwrap();

            moves.push(Move {
                dir: dir.unwrap(),
                steps,
            })
        }
    }

    moves
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Point2D {
    pub x: i32,
    pub y: i32,
}

impl Point2D {
    pub fn manhattan_distance(&self, rhs: &Point2D) -> i32 {
        (self.x - rhs.x).abs() + (self.y - rhs.y).abs()
    }

    pub fn is_diagonal_move(&self, rhs: &Point2D) -> bool {
        let dist = self.manhattan_distance(rhs);

        dist > 2 && (self.x != rhs.x && self.y != rhs.y)
    }

    pub fn is_lateral_move(&self, rhs: &Point2D) -> bool {
        let dist = self.manhattan_distance(rhs);

        dist > 1 && (self.x == rhs.x || self.y == rhs.y)
    }
}

#[derive(Debug, PartialEq)]
pub struct PlankTracker {
    knots: Vec<Point2D>,
    tail_visited_positions: HashSet<Point2D>,
}

impl PlankTracker {
    pub fn move_knots(&mut self, next_move: Move) {
        fn move_head(move_dir: MoveDir, knot: &mut Point2D) {
            match move_dir {
                MoveDir::Left => {
                    knot.x -= 1;
                }
                MoveDir::Right => {
                    knot.x += 1;
                }
                MoveDir::Up => {
                    knot.y += 1;
                }
                MoveDir::Down => {
                    knot.y -= 1;
                }
            }
        }

        fn get_move_condition(knot: Point2D, knot_next: Point2D) -> MoveCondition {
            if knot.is_lateral_move(&knot_next) {
                return MoveCondition::Lateral;
            } else if knot.is_diagonal_move(&knot_next) {
                return MoveCondition::Diagonal;
            }
            MoveCondition::Adjacent
        }

        fn move_next_knot(move_condition: MoveCondition, knot: &Point2D, knot_next: &mut Point2D) {
            // H=head, T=tail (prev), t=tail (next)
            match move_condition {
                MoveCondition::Lateral => {
                    // .....    .....
                    // .TH.. -> .TtH.
                    // .....    .....
                    if knot.x > knot_next.x + 1 {
                        knot_next.x += 1;
                    }
                    
                    // .....    .....
                    // ..HT. -> .HtT.
                    // .....    .....
                    if knot.x < knot_next.x - 1 {
                        knot_next.x -= 1;
                    }
                  
                    // ...    ...
                    // ...    .H.
                    // .H. -> .t.
                    // .T.    .T.
                    // ...    ...
                    if knot.y > knot_next.y + 1 {
                        knot_next.y += 1;
                    }
                    
                    // ...    ...
                    // .T.    .T.
                    // .H. -> .t.
                    // ...    .H.
                    // ...    ...
                    if knot.y < knot_next.y - 1 {
                        knot_next.y -= 1;
                    }  
                }
                MoveCondition::Diagonal => {
                    // ......
                    // ......
                    // .Ht...
                    // ...T..
                    // ......
                    
                    if knot.x < knot_next.x - 1 && knot.y > knot_next.y {
                        knot_next.x -= 1;
                        knot_next.y += 1;
                    }

                    // ......
                    // ......
                    // ......
                    // ...T..
                    // .Ht...

                    if knot.x < knot_next.x - 1 && knot.y < knot_next.y {
                        knot_next.x -= 1;
                        knot_next.y -= 1;
                    }

                    // ......
                    // ......
                    // ....tH
                    // ...T..
                    // ......

                    if knot.x > knot_next.x + 1 && knot.y > knot_next.y {
                        knot_next.x += 1;
                        knot_next.y += 1;
                    }

                    // ......
                    // ......
                    // ......
                    // ...T..
                    // ....tH

                    if knot.x > knot_next.x + 1 && knot.y < knot_next.y {
                        knot_next.x += 1;
                        knot_next.y -= 1;
                    }

                    // ......
                    // ..H...
                    // ..t...
                    // ...T..
                    // ......
                    
                    if knot.x < knot_next.x && knot.y > knot_next.y + 1 {
                        knot_next.x -= 1;
                        knot_next.y += 1;
                    }

                    // ......
                    // ....H.
                    // ....t.
                    // ...T..
                    // ......

                    if knot.x > knot_next.x && knot.y > knot_next.y + 1 {
                        knot_next.x += 1;
                        knot_next.y += 1;
                    }

                    // ......
                    // ......
                    // ..T...
                    // ...t..
                    // ...H..
                    
                    if knot.x > knot_next.x && knot.y < knot_next.y - 1 {
                        knot_next.x += 1;
                        knot_next.y -= 1;
                    }

                    // ......
                    // ......
                    // ..T...
                    // .t....
                    // .H....
                    
                    if knot.x < knot_next.x && knot.y < knot_next.y - 1 {
                        knot_next.x -= 1;
                        knot_next.y -= 1;
                    }
                }
                MoveCondition::Adjacent => {}
            }
        }

        for move_steps_i in 0..next_move.steps {
            println!(
                "----- START MOVE {0:?} STEP {1} -----",
                next_move.dir, move_steps_i
            );

            move_head(next_move.dir, &mut self.knots[0]);

            let mut it = self.knots.iter_mut().enumerate().peekable();
            while let Some((knots_i, knot)) = it.next() {
                if let Some((_, knot_next)) = it.peek_mut() {
                    let move_condition = get_move_condition(knot.clone(), knot_next.clone());
                    println!(
                        "Knot {0}: move condition {1:?}, knot {2:?}, knot_next {3:?}",
                        knots_i,
                        move_condition,
                        knot.clone(),
                        knot_next.clone()
                    );
                    move_next_knot(move_condition, knot, knot_next);
                } else {
                    self.tail_visited_positions.insert(knot.clone());
                }
            }

            println!("{:?}", self.knots);
            println!(
                "----- END MOVE {0:?} STEP {1} -----",
                next_move.dir, move_steps_i
            );
        }
    }
}

pub fn get_rope_tail_visits(num_knots: usize, input_file: &str) -> u32 {
    let moves = parse_input(input_file);

    let mut plank = PlankTracker {
        knots: vec![Point2D { x: 0, y: 0 }; num_knots],
        tail_visited_positions: HashSet::new(),
    };

    for next_move in moves {
        plank.move_knots(next_move);
    }

    plank.tail_visited_positions.len() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_rope_tail_visits_example() {
        assert_eq!(13, get_rope_tail_visits(2, "input/2022/day09_test_example.txt"));
    }

    #[test]
    fn test_get_rope_tail_visits_direct_left() {
        assert_eq!(
            10,
            get_rope_tail_visits(2, "input/2022/day09_test_direct_left.txt")
        );
    }

    #[test]
    fn test_get_rope_tail_visits_direct_right() {
        assert_eq!(
            1,
            get_rope_tail_visits(10, "input/2022/day09_test_direct_right.txt")
        );
    }

    #[test]
    fn test_get_rope_tail_visits_direct_up() {
        assert_eq!(2, get_rope_tail_visits(2, "input/2022/day09_test_direct_up.txt"));
    }

    #[test]
    fn test_get_rope_tail_visits_direct_down() {
        assert_eq!(
            2,
            get_rope_tail_visits(2, "input/2022/day09_test_direct_down.txt")
        );
    }

    #[test]
    fn test_get_rope_tail_visits_diagonal_down_left() {
        assert_eq!(
            2,
            get_rope_tail_visits(2, "input/2022/day09_test_diagonal_down_left.txt")
        );
    }

    #[test]
    fn test_get_rope_tail_visits_diagonal_down_right() {
        assert_eq!(
            2,
            get_rope_tail_visits(2, "input/2022/day09_test_diagonal_down_right.txt")
        );
    }

    #[test]
    fn test_get_rope_tail_visits_diagonal_left_down() {
        assert_eq!(
            2,
            get_rope_tail_visits(2, "input/2022/day09_test_diagonal_left_down.txt")
        );
    }

    #[test]
    fn test_get_rope_tail_visits_diagonal_left_up() {
        assert_eq!(
            2,
            get_rope_tail_visits(2, "input/2022/day09_test_diagonal_left_up.txt")
        );
    }

    #[test]
    fn test_get_rope_tail_visits_diagonal_right_down() {
        assert_eq!(
            2,
            get_rope_tail_visits(2, "input/2022/day09_test_diagonal_right_down.txt")
        );
    }

    #[test]
    fn test_get_rope_tail_visits_diagonal_right_up() {
        assert_eq!(
            2,
            get_rope_tail_visits(2, "input/2022/day09_test_diagonal_right_up.txt")
        );
    }

    #[test]
    fn test_get_rope_tail_visits_diagonal_up_left() {
        assert_eq!(
            2,
            get_rope_tail_visits(2, "input/2022/day09_test_diagonal_up_left.txt")
        );
    }

    #[test]
    fn test_get_rope_tail_visits_diagonal_up_right() {
        assert_eq!(
            2,
            get_rope_tail_visits(2, "input/2022/day09_test_diagonal_up_right.txt")
        );
    }

    #[test]
    fn test_get_rope_tail_visits_diagonal_then_lateral() {
        assert_eq!(
            1,
            get_rope_tail_visits(2, "input/2022/day09_test_diagonal_then_lateral.txt")
        );
    }

    #[test]
    fn test_get_rope_tail_visits_part_1() {
        assert_eq!(6044, get_rope_tail_visits(2, "input/2022/day09.txt"));
    }

    #[test]
    fn test_get_rope_tail_visits_part_2_test01() {
        assert_eq!(
            36,
            get_rope_tail_visits(10, "input/2022/day09_part02_test01.txt")
        );
    }

    #[test]
    fn test_get_rope_tail_visits_part_2_test02() {
        assert_eq!(1, get_rope_tail_visits(10, "input/2022/day09_part02_test02.txt"));
    }

    // Initial state
    // ..........................
    // ..........................
    // ..........................
    // ..........................
    // ..........................
    // ..........................
    // ..........................
    // ................H.........
    // ................1.........
    // ................2.........
    // ................3.........
    // ...............54.........
    // ..............6...........
    // .............7............
    // ............8.............
    // ...........9..............
    // ..........................
    // ..........................
    // ..........................
    // ..........................
    // ..........................

    // L 0
    // ..........................
    // ..........................
    // ..........................
    // ..........................
    // ..........................
    // ..........................
    // ..........................
    // ...............H..........
    // ................1.........
    // ................2.........
    // ................3.........
    // ...............54.........
    // ..............6...........
    // .............7............
    // ............8.............
    // ...........9..............
    // ..........................
    // ..........................
    // ..........................
    // ..........................
    // ..........................

    // L1
    // ..........................
    // ..........................
    // ..........................
    // ..........................
    // ..........................
    // ..........................
    // ..........................
    // ..............H1..........
    // ...............2..........
    // ...............3..........
    // ...............4..........
    // ...............5..........
    // ..............6...........
    // .............7............
    // ............8.............
    // ...........9..............
    // ..........................
    // ..........................
    // ..........................
    // ..........................
    // ..........................

    // L2
    // ..........................
    // ..........................
    // ..........................
    // ..........................
    // ..........................
    // ..........................
    // ..........................
    // .............H1...........
    // ...............2..........
    // ...............3..........
    // ...............4..........
    // ...............5..........
    // ..............6...........
    // .............7............
    // ............8.............
    // ...........9..............
    // ..........................
    // ..........................
    // ..........................
    // ..........................
    // ..........................

    // L3
    // ..........................
    // ..........................
    // ..........................
    // ..........................
    // ..........................
    // ..........................
    // ..........................
    // ............H12...........
    // ..............3...........
    // ..............4...........
    // ..............5...........
    // ..........................
    // ..............6...........
    // .............7............
    // ............8.............
    // ...........9..............
    // ..........................
    // ..........................
    // ..........................
    // ..........................
    // ..........................

    // ..........................
    // ..........................
    // ..........................
    // ..........................
    // ..........................
    // ..........................
    // ..........................
    // ............H12...........
    // ..............3...........
    // ..............4...........
    // ..............5...........
    // ..............6...........
    // ..............7...........
    // .............8............
    // ............9.............
    // ...........s..............
    // ..........................
    // ..........................
    // ..........................
    // ..........................
    // ..........................

    #[test]
    fn test_get_rope_tail_visits_part_2_test03() {
        assert_eq!(2, get_rope_tail_visits(10, "input/2022/day09_part02_test03.txt"));
    }

    #[test]
    fn test_get_rope_tail_visits_part_2() {
        assert_eq!(2384, get_rope_tail_visits(10, "input/2022/day09.txt"));
    }
}
