use crate::utils::{int_to_instruction, int_to_modes};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Opcode {
    Add = 1isize,
    Multiply = 2isize,
    Store = 3isize,
    Load = 4isize,
    JumpIfTrue = 5isize,
    JumpIfFalse = 6isize,
    LessThan = 7isize,
    Equals = 8isize,
    RelativeBaseOffset = 9isize,
    Halt = 99isize,
}

impl TryFrom<isize> for Opcode {
    type Error = String;

    fn try_from(value: isize) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Opcode::Add),
            2 => Ok(Opcode::Multiply),
            3 => Ok(Opcode::Store),
            4 => Ok(Opcode::Load),
            5 => Ok(Opcode::JumpIfTrue),
            6 => Ok(Opcode::JumpIfFalse),
            7 => Ok(Opcode::LessThan),
            8 => Ok(Opcode::Equals),
            9 => Ok(Opcode::RelativeBaseOffset),
            99 => Ok(Opcode::Halt),
            _ => Err(format!("Invalid opcode: {value}")),
        }
    }
}

pub fn parse_intcode_input(input_file: &str) -> Vec<isize> {
    let input = std::fs::read_to_string(input_file).expect("Failed to read file");
    input
        .trim()
        .split(',')
        .map(|s| s.trim().parse::<isize>().unwrap())
        .collect()
}

pub fn run_intcode<'a>(
    intcode: &'a mut [isize],
    prog_counter: &mut usize,
    relative_base: &mut isize,
    inputs: &mut Vec<isize>,
    outputs: &mut Vec<isize>,
) -> &'a [isize] {
    let instruction = int_to_instruction(intcode[*prog_counter]);
    let modes = int_to_modes(intcode[*prog_counter]);
    match Opcode::try_from(instruction) {
        Ok(Opcode::Add) => {
            println!("Add at position {}", *prog_counter);
            calc_add(intcode, &modes, *prog_counter, *relative_base);
            *prog_counter += 4;
            run_intcode(intcode, prog_counter, relative_base, inputs, outputs)
        }
        Ok(Opcode::Multiply) => {
            println!("Multiply at position {}", *prog_counter);
            calc_multiply(intcode, &modes, *prog_counter, *relative_base);
            *prog_counter += 4;
            run_intcode(intcode, prog_counter, relative_base, inputs, outputs)
        }
        Ok(Opcode::Store) => {
            if let Some(input) = inputs.pop() {
                println!("Store at position {}, input: {:?}", *prog_counter, input);
                calc_store(intcode, *prog_counter, input);
                *prog_counter += 2;
                run_intcode(intcode, prog_counter, relative_base, inputs, outputs)
            } else {
                println!(
                    "No input provided for Store operation at position: {}",
                    *prog_counter
                );
                intcode
            }
        }
        Ok(Opcode::Load) => {
            println!("Load at position {}", *prog_counter);
            let output = calc_load(intcode, &modes, *prog_counter, *relative_base);
            outputs.push(output);
            *prog_counter += 2;
            run_intcode(intcode, prog_counter, relative_base, inputs, outputs)
        }
        Ok(Opcode::JumpIfTrue) => {
            let maybe_jump_counter =
                calc_jump_if_true(intcode, &modes, *prog_counter, *relative_base);
            *prog_counter = if let Some(jump_counter) = maybe_jump_counter {
                jump_counter
            } else {
                *prog_counter + 3
            };
            println!("JumpIfTrue at position {}", *prog_counter);
            run_intcode(intcode, prog_counter, relative_base, inputs, outputs)
        }
        Ok(Opcode::JumpIfFalse) => {
            let maybe_jump_counter =
                calc_jump_if_false(intcode, &modes, *prog_counter, *relative_base);
            *prog_counter = if let Some(jump_counter) = maybe_jump_counter {
                jump_counter
            } else {
                *prog_counter + 3
            };
            println!("JumpIfFalse at position {}", *prog_counter);
            run_intcode(intcode, prog_counter, relative_base, inputs, outputs)
        }
        Ok(Opcode::LessThan) => {
            println!("LessThan at position {}", *prog_counter);
            calc_less_than(intcode, &modes, *prog_counter, *relative_base);
            *prog_counter += 4;
            run_intcode(intcode, prog_counter, relative_base, inputs, outputs)
        }
        Ok(Opcode::Equals) => {
            println!("Equals at position {}", *prog_counter);
            calc_equals(intcode, &modes, *prog_counter, *relative_base);
            *prog_counter += 4;
            run_intcode(intcode, prog_counter, relative_base, inputs, outputs)
        }
        Ok(Opcode::RelativeBaseOffset) => {
            println!("RelativeBaseOffset at position {}", *prog_counter);
            let relative_base_offset =
                calc_relative_base_offset(intcode, &modes, *prog_counter, *relative_base);
            *relative_base += relative_base_offset;
            intcode
        }
        Ok(Opcode::Halt) => {
            println!("Halt at position {}", *prog_counter);
            intcode
        }
        Err(_) => panic!("Unexpected Opcode {}", intcode[*prog_counter]),
    }
}

pub fn calc_add(intcode: &mut [isize], modes: &[isize], prog_counter: usize, relative_base: isize) {
    let params = get_parameters(intcode, prog_counter, 3);
    let (param_1, param_2, param_3) = (params[0], params[1], params[2]);

    let operand_lhs = get_parameter_value(
        intcode,
        param_1,
        *modes.first().unwrap_or(&0),
        relative_base,
    );
    let operand_rhs =
        get_parameter_value(intcode, param_2, *modes.get(1).unwrap_or(&0), relative_base);
    intcode[param_3 as usize] = operand_lhs + operand_rhs;
}

pub fn calc_multiply(
    intcode: &mut [isize],
    modes: &[isize],
    prog_counter: usize,
    relative_base: isize,
) {
    let params = get_parameters(intcode, prog_counter, 3);
    let (param_1, param_2, param_3) = (params[0], params[1], params[2]);

    let operand_lhs = get_parameter_value(
        intcode,
        param_1,
        *modes.first().unwrap_or(&0),
        relative_base,
    );
    let operand_rhs =
        get_parameter_value(intcode, param_2, *modes.get(1).unwrap_or(&0), relative_base);
    intcode[param_3 as usize] = operand_lhs * operand_rhs;
}

pub fn calc_store(intcode: &mut [isize], prog_counter: usize, input: isize) {
    let params = get_parameters(intcode, prog_counter, 1);
    let param_1 = params[0];

    intcode[param_1 as usize] = input;
}

pub fn calc_load(
    intcode: &mut [isize],
    modes: &[isize],
    prog_counter: usize,
    relative_base: isize,
) -> isize {
    let params = get_parameters(intcode, prog_counter, 1);
    let param_1 = params[0];

    get_parameter_value(
        intcode,
        param_1,
        *modes.first().unwrap_or(&0),
        relative_base,
    )
}

pub fn calc_jump_if_true(
    intcode: &mut [isize],
    modes: &[isize],
    prog_counter: usize,
    relative_base: isize,
) -> Option<usize> {
    let params = get_parameters(intcode, prog_counter, 2);
    let (param_1, param_2) = (params[0], params[1]);

    let operand_1 = get_parameter_value(
        intcode,
        param_1,
        *modes.first().unwrap_or(&0),
        relative_base,
    );
    let operand_2 =
        get_parameter_value(intcode, param_2, *modes.get(1).unwrap_or(&0), relative_base);
    if operand_1 != 0 {
        Some(operand_2 as usize)
    } else {
        None
    }
}

pub fn calc_jump_if_false(
    intcode: &mut [isize],
    modes: &[isize],
    prog_counter: usize,
    relative_base: isize,
) -> Option<usize> {
    let params = get_parameters(intcode, prog_counter, 2);
    let (param_1, param_2) = (params[0], params[1]);

    let operand_1 = get_parameter_value(
        intcode,
        param_1,
        *modes.first().unwrap_or(&0),
        relative_base,
    );
    let operand_2 =
        get_parameter_value(intcode, param_2, *modes.get(1).unwrap_or(&0), relative_base);
    if operand_1 == 0 {
        Some(operand_2 as usize)
    } else {
        None
    }
}

pub fn calc_less_than(
    intcode: &mut [isize],
    modes: &[isize],
    prog_counter: usize,
    relative_base: isize,
) {
    let params = get_parameters(intcode, prog_counter, 3);
    let (param_1, param_2, param_3) = (params[0], params[1], params[2]);

    let operand_lhs = get_parameter_value(
        intcode,
        param_1,
        *modes.first().unwrap_or(&0),
        relative_base,
    );
    let operand_rhs =
        get_parameter_value(intcode, param_2, *modes.get(1).unwrap_or(&0), relative_base);
    if operand_lhs < operand_rhs {
        intcode[param_3 as usize] = 1
    } else {
        intcode[param_3 as usize] = 0
    }
}

pub fn calc_equals(
    intcode: &mut [isize],
    modes: &[isize],
    prog_counter: usize,
    relative_base: isize,
) {
    let params = get_parameters(intcode, prog_counter, 3);
    let (param_1, param_2, param_3) = (params[0], params[1], params[2]);

    let operand_lhs = get_parameter_value(
        intcode,
        param_1,
        *modes.first().unwrap_or(&0),
        relative_base,
    );
    let operand_rhs =
        get_parameter_value(intcode, param_2, *modes.get(1).unwrap_or(&0), relative_base);
    if operand_lhs == operand_rhs {
        intcode[param_3 as usize] = 1
    } else {
        intcode[param_3 as usize] = 0
    }
}

pub fn calc_relative_base_offset(
    intcode: &mut [isize],
    modes: &[isize],
    prog_counter: usize,
    relative_base: isize,
) -> isize {
    let params = get_parameters(intcode, prog_counter, 1);
    let param_1 = params[0];
    let operand_1 = get_parameter_value(
        intcode,
        param_1,
        *modes.first().unwrap_or(&0),
        relative_base,
    );
    operand_1
}

// Helper function to extract parameters from intcode at given offsets
fn get_parameters(intcode: &[isize], prog_counter: usize, count: usize) -> Vec<isize> {
    (1..=count)
        .map(|offset| intcode[prog_counter + offset])
        .collect()
}

// Helper function to resolve parameter value based on mode
fn get_parameter_value(
    intcode: &[isize],
    param: isize,
    mode: isize,
    relative_base: isize,
) -> isize {
    match mode {
        0 => intcode[param as usize],                          // Position mode
        1 => param,                                            // Immediate mode
        2 => intcode[param as usize + relative_base as usize], // Relative mode
        _ => panic!("Invalid parameter mode: {mode}"),
    }
}
