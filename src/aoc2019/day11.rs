// https://adventofcode.com/2019/day/11

use std::collections::HashMap;

use crate::{
    intcode::{Opcode, run_intcode},
    utils::Direction,
};

struct RobotPose {
    pos: (isize, isize),
    dir: Direction,
}

impl RobotPose {
    pub fn new() -> RobotPose {
        RobotPose {
            pos: (0, 0),
            dir: Direction::N,
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Hash)]
#[repr(u8)]
enum Color {
    Black = 1,
    White = 2,
}

impl TryFrom<u8> for Color {
    type Error = ();

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            x if x == Color::Black as u8 => Ok(Color::Black),
            x if x == Color::White as u8 => Ok(Color::White),
            _ => Err(()),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, Default)]
#[repr(u8)]
pub enum MoveDir {
    #[default]
    Left = 0,
    Right = 1,
}

impl TryFrom<u8> for MoveDir {
    type Error = ();

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            x if x == MoveDir::Left as u8 => Ok(MoveDir::Left),
            x if x == MoveDir::Right as u8 => Ok(MoveDir::Right),
            _ => Err(()),
        }
    }
}

pub fn get_painted_panels(intcode: &mut [isize]) -> usize {
    let mut outputs = vec![];
    let mut grid: HashMap<(isize, isize), Color> = HashMap::new();

    let mut prog_counter: usize = 0usize;

    let mut pose = RobotPose::new();

    // Robot starts at initial position on black grid square
    grid.insert((0, 0), Color::Black);

    loop {
        if let Some(color) = grid.get(&pose.pos) {
            println!("*****");
            println!("Robot is at {:?}, color is {:?}", pose.pos, color);
            println!("");

            let input_value: isize = *color as u8 as isize;

            let mut inputs: Vec<isize> = vec![input_value];

            run_intcode(
                intcode,
                &mut prog_counter,
                &mut 0,
                &mut inputs,
                &mut outputs,
            );

            let last_opcode = intcode[prog_counter];
            if last_opcode == Opcode::Halt as isize {
                break;
            }

            if outputs.len() != 2 {
                panic!("Received invalid number of outputs from intcode")
            }

            if let Some(output_1) = outputs.first() {
                if let Ok(grid_color) = Color::try_from(*output_1 as u8) {
                    grid.insert(pose.pos, grid_color);
                } else {
                    panic!("Invalid grid color");
                }
            } else {
                panic!("Could not retrieve output 1");
            }

            if let Some(output_2) = outputs.get(1) {
                if let Ok(move_dir) = MoveDir::try_from(*output_2 as u8) {
                    let next_dir = match move_dir {
                        MoveDir::Left => pose.dir.turn_left(),
                        MoveDir::Right => pose.dir.turn_right(),
                    };
                    let (x, y) = pose.pos;
                    let (dx, dy) = next_dir.to_delta();
                    let next_pos = (x + dx, y + dy);
                    pose.pos = next_pos;
                    pose.dir = next_dir;
                    grid.insert(next_pos, Color::Black);
                } else {
                    panic!("Invalid move dir");
                }
            } else {
                panic!("Could not retrieve output 2");
            }
        } else {
            panic!("Invalid robot pos");
        }

        println!("*****");
        println!("");
    }

    0
}

#[cfg(test)]
mod tests {
    use crate::aoc2019::day11::get_painted_panels;
    use crate::intcode::parse_intcode_input;

    #[test]
    fn test_run_intcode() {
        let mut input_intcode = parse_intcode_input("input/2019/day11.txt");
        input_intcode.extend(vec![0; 1000]);

        let painted_panels = get_painted_panels(&mut input_intcode);
        assert_eq!(0, painted_panels);
    }
}
