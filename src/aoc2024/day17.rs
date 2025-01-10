// https://adventofcode.com/2024/day/17

use crate::utils::get_lines;

struct Registers {
    reg_a: u64,
    reg_b: u64,
    reg_c: u64,
}

impl Registers {
    fn new() -> Registers {
        Registers {
            reg_a: 0,
            reg_b: 0,
            reg_c: 0,
        }
    }
}

struct Input {
    registers: Registers,
    program: Vec<u8>,
}

fn parse_input(input_file: &str) -> Input {
    let lines = get_lines(input_file);

    let mut registers: Registers = Registers::new();
    let mut program = vec![];

    for line in lines {
        let mut parts = line.splitn(2, ":");
        if let (Some(part_a), Some(part_b)) = (parts.next(), parts.next()) {
            let parts: Vec<&str> = part_a.split_whitespace().collect();
            if let [_, reg] = parts.as_slice() {
                match *reg {
                    "A" => registers.reg_a = part_b.trim().parse().unwrap_or(0),
                    "B" => registers.reg_b = part_b.trim().parse().unwrap_or(0),
                    "C" => registers.reg_c = part_b.trim().parse().unwrap_or(0),
                    _ => {}
                }
            } else {
                program = part_b.split(",").map(|o| o.parse().unwrap_or(0)).collect();
            }
        }
    }

    Input { registers, program }
}

fn get_joined_vals(input_file: &str) -> String {
    let input = parse_input(input_file);

    let mut reg_a = input.registers.reg_a;
    let mut reg_b = input.registers.reg_b;
    let mut reg_c = input.registers.reg_c;

    println!("Reg A: {:?}, B: {:?}, C: {:?}", reg_a, reg_b, reg_c);

    let program = input.program;

    println!("program {:?}", program);

    let joined_vals = String::new();

    let mut ins_ptr = 0usize;

    loop {
        if let Some(opcode) = program.get(ins_ptr) {
            match opcode {
                // adv
                0 => {

                },
                // bxl
                1 => {

                },
                // bst
                2 => {
                    
                },
                // jnz
                3 => {

                },
                // bxc
                4 => {

                }
                // out
                5 => {

                }
                // bdv
                6 => {

                }
                // cdv
                7 => {

                }
                _ => {
                    panic!("unknown opcode {}", opcode);
                }
            }
        }
    }

    joined_vals
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_joined_vals_test01() {
        assert_eq!("", get_joined_vals("input/2024/day17_test01.txt"));
    }

    #[test]
    fn test_get_joined_vals() {
        assert_eq!("", get_joined_vals("input/2024/day17.txt"));
    }
}
