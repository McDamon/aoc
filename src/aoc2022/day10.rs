// https://adventofcode.com/2022/day/10

use crate::utils::get_lines;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum InstructionType {
    Noop,
    Addx,
}

#[derive(Debug, PartialEq)]
pub struct Instruction {
    instruction_type: InstructionType,
    instruction_val: Option<i32>,
}

pub fn parse_input(input_file: &str) -> Vec<Instruction> {
    let mut moves: Vec<Instruction> = vec![];

    let lines = get_lines(input_file);

    for line in lines {
        let split_line: Vec<&str> = line.split(' ').collect();

        let instruction_type = match split_line[0] {
            "noop" => Some(InstructionType::Noop),
            "addx" => Some(InstructionType::Addx),
            _ => None,
        };

        let mut instruction_val = None;

        if split_line.len() > 1 {
            instruction_val = Some(split_line[1].parse().unwrap());
        }

        moves.push(Instruction {
            instruction_type: instruction_type.unwrap(),
            instruction_val,
        });
    }

    moves
}

pub fn get_sum_signal_strengths(input_file: &str) -> i32 {
    let instructions = parse_input(input_file);

    let mut x = 1;

    let mut cycle = 1;

    let mut signal_strength_sum = 0;

    for instruction in instructions {
        match instruction.instruction_type {
            InstructionType::Noop => {
                let signal_strength = get_signal_strength(cycle, x);
                signal_strength_sum += signal_strength;
                render_cycle(cycle, x);
                cycle += 1;
            }
            InstructionType::Addx => {
                for i in 0..2 {
                    let signal_strength = get_signal_strength(cycle, x);
                    signal_strength_sum += signal_strength;
                    if i == 1
                        && let Some(val) = instruction.instruction_val
                    {
                        x += val;
                    }
                    render_cycle(cycle, x);
                    cycle += 1;
                }
            }
        }
    }

    signal_strength_sum
}

fn render_cycle(cycle: i32, x: i32) {
    let offset = 40 * (cycle / 40);
    if cycle == x + offset - 1 || cycle == x + offset || cycle == x + offset + 1 {
        print!("#");
    } else {
        print!(".");
    }
    if cycle % 40 == 0 {
        println!();
    }
}

fn get_signal_strength(cycle: i32, x: i32) -> i32 {
    let mut signal_strength = 0;
    if (cycle == 20) || ((cycle + 20) % 40 == 0) {
        signal_strength = cycle * x;
    }
    signal_strength
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_sum_signal_strengths_test01() {
        assert_eq!(0, get_sum_signal_strengths("input/2022/day10_test01.txt"));
    }

    #[test]
    fn test_get_sum_signal_strengths_test02() {
        assert_eq!(
            13140,
            get_sum_signal_strengths("input/2022/day10_test02.txt")
        );
    }

    #[test]
    fn test_get_sum_signal_strengths() {
        assert_eq!(12520, get_sum_signal_strengths("input/2022/day10.txt"));
    }
}
