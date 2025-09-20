// https://adventofcode.com/2019/day/11

use core::panic;
use std::collections::HashMap;

use crate::{
    intcode::{Opcode, run_intcode},
    utils::Direction,
};

pub struct RobotPose {
    pos: (isize, isize),
    dir: Direction,
}

impl Default for RobotPose {
    fn default() -> Self {
        Self::new()
    }
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
pub enum Color {
    Black = 0,
    White = 1,
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

pub fn run_painted_panels(
    intcode: &mut [isize],
    prog_counter: &mut usize,
    inputs: &mut Vec<isize>,
) -> Result<(Color, MoveDir), Opcode> {
    let mut outputs = vec![];

    run_intcode(intcode, prog_counter, &mut 0, inputs, &mut outputs);

    if let Ok(last_opcode) = Opcode::try_from(intcode[*prog_counter]) {
        if last_opcode == Opcode::Halt {
            return Err(last_opcode);
        }

        if let Some(output_1) = outputs.first()
            && let Some(output_2) = outputs.get(1)
        {
            if let Ok(grid_color) = Color::try_from(*output_1 as u8)
                && let Ok(move_dir) = MoveDir::try_from(*output_2 as u8)
            {
                Ok((grid_color, move_dir))
            } else {
                panic!("Invalid grid_color and move_dir");
            }
        } else {
            Err(last_opcode)
        }
    } else {
        panic!("Invalid opcode");
    }
}

pub fn get_painted_panels(intcode: &mut [isize]) -> usize {
    let mut grid: HashMap<(isize, isize), Color> = HashMap::new();

    // Robot starts at initial position on black grid square
    grid.insert((0, 0), Color::Black);

    let mut pose = RobotPose::new();

    let mut prog_counter = 0usize;

    let mut inputs = vec![];

    loop {
        let result = run_painted_panels(intcode, &mut prog_counter, &mut inputs);
        match result {
            Ok((grid_color, move_dir)) => {
                grid.insert(pose.pos, grid_color);

                let next_dir = match move_dir {
                    MoveDir::Left => pose.dir.turn_left(),
                    MoveDir::Right => pose.dir.turn_right(),
                };
                let (x, y) = pose.pos;
                let (dx, dy) = next_dir.to_delta();
                let next_pos = (x + dx, y + dy);
                pose.pos = next_pos;
                pose.dir = next_dir;
                grid.entry(next_pos).or_insert(Color::Black);
            }
            Err(opcode) => match opcode {
                Opcode::Store => {
                    if let Some(color) = grid.get(&pose.pos) {
                        let input_value: isize = *color as u8 as isize;
                        inputs.push(input_value);
                    }
                }
                Opcode::Halt => break,
                _ => (),
            },
        }
    }

    grid.len()
}

#[cfg(test)]
mod tests {
    use crate::aoc2019::day11::get_painted_panels;
    use crate::intcode::parse_intcode_input;

    #[test]
    fn test_get_painted_panels() {
        let mut input_intcode = parse_intcode_input("input/2019/day11.txt");
        input_intcode.extend(vec![0; 1000]);

        let painted_panels = get_painted_panels(&mut input_intcode);
        assert_eq!(2293, painted_panels);
    }
}
