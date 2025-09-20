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

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Mode {
    Position = 0isize,
    Immediate = 1isize,
    Relative = 2isize,
}

impl TryFrom<isize> for Mode {
    type Error = String;

    fn try_from(value: isize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Mode::Position),
            1 => Ok(Mode::Immediate),
            2 => Ok(Mode::Relative),
            _ => Err(format!("Invalid mode: {value}")),
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
                calc_store(intcode, &modes, *prog_counter, *relative_base, input);
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
            let relative_base_offset =
                calc_relative_base_offset(intcode, &modes, *prog_counter, *relative_base);
            *relative_base += relative_base_offset;
            println!(
                "RelativeBaseOffset at position {}, new relative base {}",
                *prog_counter, *relative_base
            );
            *prog_counter += 2;
            run_intcode(intcode, prog_counter, relative_base, inputs, outputs)
        }
        Ok(Opcode::Halt) => {
            println!("Halt at position {}", *prog_counter);
            intcode
        }
        Err(_) => panic!("Unexpected Opcode {}", intcode[*prog_counter]),
    }
}

pub fn calc_add(intcode: &mut [isize], modes: &[isize], prog_counter: usize, relative_base: isize) {
    let (operand_lhs, operand_rhs, dest_param, dest_mode) =
        get_two_operands_and_dest(intcode, modes, prog_counter, relative_base);
    write_parameter(
        intcode,
        dest_param,
        dest_mode,
        relative_base,
        operand_lhs + operand_rhs,
    );
}

pub fn calc_multiply(
    intcode: &mut [isize],
    modes: &[isize],
    prog_counter: usize,
    relative_base: isize,
) {
    let (operand_lhs, operand_rhs, dest_param, dest_mode) =
        get_two_operands_and_dest(intcode, modes, prog_counter, relative_base);
    write_parameter(
        intcode,
        dest_param,
        dest_mode,
        relative_base,
        operand_lhs * operand_rhs,
    );
}

pub fn calc_store(
    intcode: &mut [isize],
    modes: &[isize],
    prog_counter: usize,
    relative_base: isize,
    input: isize,
) {
    let params = get_parameters(intcode, prog_counter, 1);
    let param_1 = params[0];
    let mode_1 = get_mode(*modes.first().unwrap_or(&0));
    // write input into the destination parameter (param_1) according to its mode
    write_parameter(intcode, param_1, mode_1, relative_base, input);
}

pub fn calc_load(
    intcode: &mut [isize],
    modes: &[isize],
    prog_counter: usize,
    relative_base: isize,
) -> isize {
    let params = get_parameters(intcode, prog_counter, 1);
    let param_1 = params[0];

    let mode = get_mode(*modes.first().unwrap_or(&0));
    get_parameter_value(intcode, param_1, mode, relative_base)
}

pub fn calc_jump_if_true(
    intcode: &mut [isize],
    modes: &[isize],
    prog_counter: usize,
    relative_base: isize,
) -> Option<usize> {
    let params = get_parameters(intcode, prog_counter, 2);
    let (param_1, param_2) = (params[0], params[1]);

    let mode_1 = get_mode(*modes.first().unwrap_or(&0));
    let operand_1 = get_parameter_value(intcode, param_1, mode_1, relative_base);
    let mode_2 = get_mode(*modes.get(1).unwrap_or(&0));
    let operand_2 = get_parameter_value(intcode, param_2, mode_2, relative_base);
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

    let mode_1 = get_mode(*modes.first().unwrap_or(&0));
    let operand_1 = get_parameter_value(intcode, param_1, mode_1, relative_base);
    let mode_2 = get_mode(*modes.get(1).unwrap_or(&0));
    let operand_2 = get_parameter_value(intcode, param_2, mode_2, relative_base);
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
    let (operand_lhs, operand_rhs, dest_param, dest_mode) =
        get_two_operands_and_dest(intcode, modes, prog_counter, relative_base);
    let result = if operand_lhs < operand_rhs { 1 } else { 0 };
    write_parameter(intcode, dest_param, dest_mode, relative_base, result);
}

pub fn calc_equals(
    intcode: &mut [isize],
    modes: &[isize],
    prog_counter: usize,
    relative_base: isize,
) {
    let (operand_lhs, operand_rhs, dest_param, dest_mode) =
        get_two_operands_and_dest(intcode, modes, prog_counter, relative_base);
    let result = if operand_lhs == operand_rhs { 1 } else { 0 };
    write_parameter(intcode, dest_param, dest_mode, relative_base, result);
}

pub fn calc_relative_base_offset(
    intcode: &mut [isize],
    modes: &[isize],
    prog_counter: usize,
    relative_base: isize,
) -> isize {
    let params = get_parameters(intcode, prog_counter, 1);
    let param_1 = params[0];
    let mode = get_mode(*modes.first().unwrap_or(&0));
    get_parameter_value(intcode, param_1, mode, relative_base)
}

// Helper function to extract mode
fn get_mode(mode_val: isize) -> Mode {
    match Mode::try_from(mode_val) {
        Ok(mode) => mode,
        Err(_) => panic!("Unexpected Mode {}", mode_val),
    }
}

// Helper function to extract parameters from intcode at given offsets
fn get_parameters(intcode: &[isize], prog_counter: usize, count: usize) -> Vec<isize> {
    (1..=count)
        .map(|offset| intcode[prog_counter + offset])
        .collect()
}

// Helper function to resolve parameter value based on mode
fn get_parameter_value(intcode: &[isize], param: isize, mode: Mode, relative_base: isize) -> isize {
    match mode {
        Mode::Position => {
            let index = param as usize;
            if let Some(param_val) = intcode.get(index) {
                println!("Position mode. Index {}. Param Val {}", index, param_val);
                *param_val
            } else {
                panic!("Requesting index {} larger than memory!", index);
            }
        }
        Mode::Immediate => {
            println!("Immediate mode. Param Val {}", param);
            param
        }
        Mode::Relative => {
            let index = param + relative_base;
            if index >= 0 {
                if let Some(param_val) = intcode.get(index as usize) {
                    println!("Relative mode. Index {}. Param Val {}", index, param_val);
                    *param_val
                } else {
                    panic!("Requesting index {} larger than memory!", index);
                }
            } else {
                panic!("Relative mode. Index is less than 0!");
            }
        }
    }
}

// Helper to read two operands and the destination parameter (and its mode)
fn get_two_operands_and_dest(
    intcode: &[isize],
    modes: &[isize],
    prog_counter: usize,
    relative_base: isize,
) -> (isize, isize, isize, Mode) {
    let params = get_parameters(intcode, prog_counter, 3);
    let (param_1, param_2, param_3) = (params[0], params[1], params[2]);

    let mode_lhs = get_mode(*modes.first().unwrap_or(&0));
    let operand_lhs = get_parameter_value(intcode, param_1, mode_lhs, relative_base);
    let mode_rhs = get_mode(*modes.get(1).unwrap_or(&0));
    let operand_rhs = get_parameter_value(intcode, param_2, mode_rhs, relative_base);
    let dest_mode = get_mode(*modes.get(2).unwrap_or(&0));
    (operand_lhs, operand_rhs, param_3, dest_mode)
}

// Helper to write a value into a parameter respecting its mode
fn write_parameter(
    intcode: &mut [isize],
    param: isize,
    mode: Mode,
    relative_base: isize,
    value: isize,
) {
    match mode {
        Mode::Position => {
            if param >= 0 {
                intcode[param as usize] = value;
            }
        }
        Mode::Immediate => {
            // immediate mode should not be used as a destination; ignore
        }
        Mode::Relative => {
            let index = param + relative_base;
            if index >= 0 {
                intcode[index as usize] = value;
            }
        }
    }
}
