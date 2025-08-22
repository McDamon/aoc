// https://adventofcode.com/2024/day/17

use std::{collections::HashSet, panic};

use crate::utils::get_lines;

#[derive(Clone, Copy, Debug)]
pub struct Registers {
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
    program: Vec<u64>,
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
                program = part_b
                    .split(",")
                    .map(|o| o.trim().parse().unwrap_or(0))
                    .collect();
            }
        }
    }

    Input { registers, program }
}

fn get_combo_operand_val(registers: &Registers, combo: u64) -> Option<u64> {
    match combo {
        0_u64..=3_u64 => Some(combo),
        4 => Some(registers.reg_a),
        5 => Some(registers.reg_b),
        6 => Some(registers.reg_c),
        _ => None,
    }
}

fn run_program(registers: &mut Registers, program: &[u64]) -> (Registers, Vec<u64>) {
    let mut out_vals: Vec<u64> = vec![];

    let mut ins_ptr = 0usize;

    while let Some(opcode) = program.get(ins_ptr)
        && let Some(operand) = program.get(ins_ptr + 1)
    {
        match opcode {
            // adv
            0 => {
                // adv does division
                // numerator is the value in reg_a
                // denominator is the 2** the combo operand
                let combo = get_combo_operand_val(registers, *operand).unwrap() as f32;
                let numer = registers.reg_a as f32;
                let denom = 2.0f32.powf(combo);
                let div_res = numer / denom;
                registers.reg_a = div_res as u64;

                //println!("adv, reg_a: {:?}", registers.reg_a);

                // increment ins_ptr by 2
                ins_ptr += 2;
            }
            // bxl
            1 => {
                // bxl does bitwise xor of reg_b and the literal operand
                registers.reg_b ^= *operand;

                //println!("bxl, reg_b: {:?}", registers.reg_b);

                // increment ins_ptr by 2
                ins_ptr += 2;
            }
            // bst
            2 => {
                // bst does combo operand modulo 8
                registers.reg_b = get_combo_operand_val(registers, *operand).unwrap() % 8;

                //println!("bst, reg_b: {:?}", registers.reg_b);

                // increment ins_ptr by 2
                ins_ptr += 2;
            }
            // jnz
            3 => {
                // jnz does nothing if reg_a is 0
                // If reg_a is not 0, then it jumps ins_ptr to the value of the literal operand
                if registers.reg_a != 0 {
                    ins_ptr = *operand as usize;
                    //println!("jnz, ins_ptr: {:?}", ins_ptr);
                } else {
                    //println!("ignoring jnz, ins_ptr: {:?}", ins_ptr);

                    // increment ins_ptr by 2
                    ins_ptr += 2;
                }
            }
            // bxc
            4 => {
                // bxc does bitwise xor of reg_b and reg_c
                registers.reg_b ^= registers.reg_c;

                //println!("bxc, reg_b: {:?}", registers.reg_b);

                // increment ins_ptr by 2
                ins_ptr += 2;
            }
            // out
            5 => {
                // out does combo operand modulo 8, and outputs the result
                let out_val = get_combo_operand_val(registers, *operand).unwrap() % 8;
                out_vals.push(out_val);

                //println!("out, out_val: {:?}", out_val);

                // increment ins_ptr by 2
                ins_ptr += 2;
            }
            // bdv
            6 => {
                // bdv works like adv, but result is stored in reg_b
                registers.reg_b = registers.reg_a
                    / 2_u64.pow(get_combo_operand_val(registers, *operand).unwrap() as u32);

                //println!("bdv, reg_b: {:?}", registers.reg_b);

                // increment ins_ptr by 2
                ins_ptr += 2;
            }
            // cdv
            7 => {
                // cdv works like adv, but result is stored in reg_c
                registers.reg_c = registers.reg_a
                    / 2_u64.pow(get_combo_operand_val(registers, *operand).unwrap() as u32);

                //println!("cdv, reg_c: {:?}", registers.reg_c);

                // increment ins_ptr by 2
                ins_ptr += 2;
            }
            _ => {
                panic!("unknown opcode {opcode}");
            }
        }
    }

    (*registers, out_vals)
}

fn out_vals_to_str(out_vals: &[u64]) -> String {
    out_vals
        .iter()
        .map(|v| v.to_string())
        .collect::<Vec<String>>()
        .join(",")
}

pub fn get_joined_vals(input_file: &str) -> (Registers, String) {
    let input = parse_input(input_file);

    let mut registers = input.registers;

    let program = input.program;

    println!("program {program:?}");

    let (out_registers, out_vals) = run_program(&mut registers, &program);

    (out_registers, out_vals_to_str(&out_vals))
}

pub fn get_lowest_positive_reg_a(input_file: &str) -> u64 {
    let input = parse_input(input_file);

    let mut poss_reg_a_vals: HashSet<u64> = HashSet::new();

    poss_reg_a_vals.insert(0);

    for num in input.program.iter().rev() {
        let mut new_poss_reg_a_vals = HashSet::new();

        for poss_reg_a in &poss_reg_a_vals {
            for opt in 0..8_u64 {
                let poss_reg_a = (poss_reg_a << 3) + opt;
                //println!("poss_reg_a: {:#b}", poss_reg_a);

                let mut registers = input.registers;
                registers.reg_a = poss_reg_a;
                registers.reg_b = 0;
                registers.reg_c = 0;

                let (_out_registers, out_vals) = run_program(&mut registers, &input.program);
                //println!("out_registers: {:?}", out_registers);
                //println!("out_vals: {:?}", out_vals);

                if let Some(first_out_vals) = out_vals.first()
                    && num == first_out_vals
                {
                    new_poss_reg_a_vals.insert(poss_reg_a);
                }
            }
        }
        poss_reg_a_vals = new_poss_reg_a_vals;
    }

    *poss_reg_a_vals.iter().min().unwrap_or(&0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_joined_vals_test01() {
        let (_, joined_vals) = get_joined_vals("input/2024/day17_test01.txt");
        assert_eq!("4,6,3,5,6,3,5,2,1,0", joined_vals);
    }

    #[test]
    fn test_get_joined_vals_test02() {
        let (registers, _) = get_joined_vals("input/2024/day17_test02.txt");
        assert_eq!(1, registers.reg_b);
    }

    #[test]
    fn test_get_joined_vals_test03() {
        let (_, joined_vals) = get_joined_vals("input/2024/day17_test03.txt");
        assert_eq!("0,1,2", joined_vals);
    }

    #[test]
    fn test_get_joined_vals_test04() {
        let (registers, joined_vals) = get_joined_vals("input/2024/day17_test04.txt");
        assert_eq!(0, registers.reg_a);
        assert_eq!("4,2,5,6,7,7,7,7,3,1,0", joined_vals);
    }

    #[test]
    fn test_get_joined_vals_test05() {
        let (registers, _) = get_joined_vals("input/2024/day17_test05.txt");
        assert_eq!(26, registers.reg_b);
    }

    #[test]
    fn test_get_joined_vals_test06() {
        let (registers, _) = get_joined_vals("input/2024/day17_test06.txt");
        assert_eq!(44354, registers.reg_b);
    }

    #[test]
    fn test_get_joined_vals() {
        let (_, joined_vals) = get_joined_vals("input/2024/day17.txt");
        assert_eq!("2,3,4,7,5,7,3,0,7", joined_vals);
    }

    #[test]
    fn test_get_lowest_positive_reg_a_test01() {
        assert_eq!(
            117440,
            get_lowest_positive_reg_a("input/2024/day17_test07.txt")
        );
    }

    #[test]
    fn test_get_lowest_positive_reg_a() {
        assert_eq!(
            190384609508367,
            get_lowest_positive_reg_a("input/2024/day17.txt")
        );
    }
}
