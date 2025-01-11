// https://adventofcode.com/2024/day/17

use std::panic;

use crate::utils::get_lines;

struct Registers {
    reg_a: u32,
    reg_b: u32,
    reg_c: u32,
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
                program = part_b.split(",").map(|o| o.trim().parse().unwrap_or(0)).collect();
            }
        }
    }

    Input { registers, program }
}

fn get_combo_operand_val(registers: &Registers, combo: u8) -> u32 {
    match combo {
        0_u8..=3_u8 => combo as u32,
        4 => registers.reg_a,
        5 => registers.reg_b,
        6 => registers.reg_c,
        _ => panic!("unknown combo operand {}", combo),
    }
}

fn get_joined_vals(input_file: &str) -> (Registers, String) {
    let input = parse_input(input_file);

    let mut registers = input.registers;

    println!(
        "Reg A: {:?}, B: {:?}, C: {:?}",
        registers.reg_a, registers.reg_b, registers.reg_c
    );

    let program = input.program;

    println!("program {:?}", program);

    let mut out_vals: Vec<u32> = vec![];

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
                let denom = 2.0f32.powf(get_combo_operand_val(&registers, *operand) as f32);
                let div_res = registers.reg_a as f32 / denom;
                registers.reg_a = div_res as u32;

                println!("adv, denom: {:?}, reg_a: {:?}", denom, registers.reg_a);

                // increment ins_ptr by 2
                ins_ptr += 2;
            }
            // bxl
            1 => {
                // bxl does bitwise xor of reg_b and the literal operand
                registers.reg_b = registers.reg_b ^ *operand as u32;

                println!("bxl, reg_b: {:?}", registers.reg_b);

                // increment ins_ptr by 2
                ins_ptr += 2;
            }
            // bst
            2 => {
                // bst does combo operand modulo 8
                registers.reg_b = get_combo_operand_val(&registers, *operand) % 8;

                println!("bst, reg_b: {:?}", registers.reg_b);

                // increment ins_ptr by 2
                ins_ptr += 2;
            }
            // jnz
            3 => {
                // jnz does nothing if reg_a is 0
                // If reg_a is not 0, then it jumps ins_ptr to the value of the literal operand
                if registers.reg_a != 0 {
                    ins_ptr = *operand as usize;
                    println!("jnz, ins_ptr: {:?}", ins_ptr);
                }
                else {
                    // increment ins_ptr by 2
                    ins_ptr += 2;
                }
            }
            // bxc
            4 => {
                // bxc does bitwise xor of reg_b and reg_c
                registers.reg_b = registers.reg_b ^ registers.reg_c;

                println!("bxc, reg_b: {:?}", registers.reg_b);

                // increment ins_ptr by 2
                ins_ptr += 2;
            }
            // out
            5 => {
                // out does combo operand modulo 8, and outputs the result
                let out_val = get_combo_operand_val(&registers, *operand) % 8;
                out_vals.push(out_val);

                println!("out, out_val: {:?}", out_val);

                // increment ins_ptr by 2
                ins_ptr += 2;
            }
            // bdv
            6 => {
                // bdv works like adv, but result is stored in reg_b
                registers.reg_b =
                    registers.reg_a / 2_u32.pow(get_combo_operand_val(&registers, *operand) as u32);

                println!("bdv, reg_b: {:?}", registers.reg_b);

                // increment ins_ptr by 2
                ins_ptr += 2;
            }
            // cdv
            7 => {
                // cdv works like adv, but result is stored in reg_c
                registers.reg_c =
                    registers.reg_a / 2_u32.pow(get_combo_operand_val(&registers, *operand) as u32);

                println!("cdv, reg_c: {:?}", registers.reg_c);

                // increment ins_ptr by 2
                ins_ptr += 2;
            }
            _ => {
                panic!("unknown opcode {}", opcode);
            }
        }

        println!(
            "Reg A: {:?}, B: {:?}, C: {:?}, ins_ptr: {:?}",
            registers.reg_a, registers.reg_b, registers.reg_c, ins_ptr
        );
    }

    (
        registers,
        out_vals
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(","),
    )
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
}
