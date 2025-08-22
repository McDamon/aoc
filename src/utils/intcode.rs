use crate::utils::{int_to_instruction, int_to_modes};

#[derive(Debug)]
pub enum Opcode {
    Add = 1isize,
    Multiply = 2isize,
    Store = 3isize,
    Load = 4isize,
    JumpIfTrue = 5isize,
    JumpIfFalse = 6isize,
    LessThan = 7isize,
    Equals = 8isize,
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
        .map(|s| s.parse::<isize>().unwrap())
        .collect()
}

pub fn run_intcode<'a>(
    intcode: &'a mut [isize],
    prog_counter: usize,
    input: Option<isize>,
    outputs: &mut Vec<isize>,
) -> &'a [isize] {
    let instruction = int_to_instruction(intcode[prog_counter]);
    let modes = int_to_modes(intcode[prog_counter]);
    match Opcode::try_from(instruction) {
        Ok(Opcode::Add) => {
            calc_add(intcode, &modes, prog_counter);
            run_intcode(intcode, prog_counter + 4, input, outputs)
        }
        Ok(Opcode::Multiply) => {
            calc_multiply(intcode, &modes, prog_counter);
            run_intcode(intcode, prog_counter + 4, input, outputs)
        }
        Ok(Opcode::Store) => {
            calc_store(intcode, prog_counter, input);
            run_intcode(intcode, prog_counter + 2, input, outputs)
        }
        Ok(Opcode::Load) => {
            let output = calc_load(intcode, &modes, prog_counter);
            outputs.push(output);
            run_intcode(intcode, prog_counter + 2, input, outputs)
        }
        Ok(Opcode::JumpIfTrue) => {
            let jump_counter = calc_jump_if_true(intcode, &modes, prog_counter);
            run_intcode(
                intcode,
                jump_counter.unwrap_or((prog_counter + 2) as isize) as usize,
                input,
                outputs,
            )
        }
        Ok(Opcode::JumpIfFalse) => {
            let jump_counter = calc_jump_if_false(intcode, &modes, prog_counter);
            run_intcode(
                intcode,
                jump_counter.unwrap_or((prog_counter + 2) as isize) as usize,
                input,
                outputs,
            )
        }
        Ok(Opcode::LessThan) => {
            calc_less_than(intcode, &modes, prog_counter);
            run_intcode(intcode, prog_counter + 4, input, outputs)
        }
        Ok(Opcode::Equals) => {
            calc_equals(intcode, &modes, prog_counter);
            run_intcode(intcode, prog_counter + 4, input, outputs)
        }
        Ok(Opcode::Halt) => intcode,
        Err(_) => panic!("Unex`pected Opcode {}", intcode[prog_counter]),
    }
}

pub fn calc_add(intcode: &mut [isize], modes: &[isize], prog_counter: usize) {
    let params = get_parameters(intcode, prog_counter, 3);
    let (param_1, param_2, param_3) = (params[0], params[1], params[2]);
    let operand_lhs = get_parameter_value(intcode, param_1, *modes.first().unwrap_or(&0));
    let operand_rhs = get_parameter_value(intcode, param_2, *modes.get(1).unwrap_or(&0));
    intcode[param_3 as usize] = operand_lhs + operand_rhs;
}

pub fn calc_multiply(intcode: &mut [isize], modes: &[isize], prog_counter: usize) {
    let params = get_parameters(intcode, prog_counter, 3);
    let (param_1, param_2, param_3) = (params[0], params[1], params[2]);
    let operand_lhs = get_parameter_value(intcode, param_1, *modes.first().unwrap_or(&0));
    let operand_rhs = get_parameter_value(intcode, param_2, *modes.get(1).unwrap_or(&0));
    intcode[param_3 as usize] = operand_lhs * operand_rhs;
}

pub fn calc_store(intcode: &mut [isize], prog_counter: usize, input: Option<isize>) {
    let params = get_parameters(intcode, prog_counter, 1);
    let param_1 = params[0];
    intcode[param_1 as usize] = match input {
        Some(value) => value,
        None => panic!("No input provided for Store operation"),
    };
}

pub fn calc_load(intcode: &mut [isize], modes: &[isize], prog_counter: usize) -> isize {
    let params = get_parameters(intcode, prog_counter, 1);
    let param_1 = params[0];

    get_parameter_value(intcode, param_1, *modes.first().unwrap_or(&0))
}

pub fn calc_jump_if_true(
    intcode: &mut [isize],
    modes: &[isize],
    prog_counter: usize,
) -> Option<isize> {
    let params = get_parameters(intcode, prog_counter, 2);
    let (param_1, param_2) = (params[0], params[1]);

    let test_value = get_parameter_value(intcode, param_1, *modes.first().unwrap_or(&0));
    if test_value != 0 {
        Some(get_parameter_value(
            intcode,
            param_2,
            *modes.get(1).unwrap_or(&0),
        ))
    } else {
        None
    }
}

pub fn calc_jump_if_false(
    intcode: &mut [isize],
    modes: &[isize],
    prog_counter: usize,
) -> Option<isize> {
    let params = get_parameters(intcode, prog_counter, 2);
    let (param_1, param_2) = (params[0], params[1]);

    let test_value = get_parameter_value(intcode, param_1, *modes.first().unwrap_or(&0));
    if test_value == 0 {
        Some(get_parameter_value(
            intcode,
            param_2,
            *modes.get(1).unwrap_or(&0),
        ))
    } else {
        None
    }
}

pub fn calc_less_than(intcode: &mut [isize], modes: &[isize], prog_counter: usize) {
    let params = get_parameters(intcode, prog_counter, 3);
    let (param_1, param_2, param_3) = (params[0], params[1], params[2]);
    let operand_lhs = get_parameter_value(intcode, param_1, *modes.first().unwrap_or(&0));
    let operand_rhs = get_parameter_value(intcode, param_2, *modes.get(1).unwrap_or(&0));
    if operand_lhs < operand_rhs {
        intcode[param_3 as usize] = 1
    } else {
        intcode[param_3 as usize] = 0
    }
}

pub fn calc_equals(intcode: &mut [isize], modes: &[isize], prog_counter: usize) {
    let params = get_parameters(intcode, prog_counter, 3);
    let (param_1, param_2, param_3) = (params[0], params[1], params[2]);
    let operand_lhs = get_parameter_value(intcode, param_1, *modes.first().unwrap_or(&0));
    let operand_rhs = get_parameter_value(intcode, param_2, *modes.get(1).unwrap_or(&0));
    if operand_lhs == operand_rhs {
        intcode[param_3 as usize] = 1
    } else {
        intcode[param_3 as usize] = 0
    }
}

// Helper function to extract parameters from intcode at given offsets
fn get_parameters(intcode: &[isize], prog_counter: usize, count: usize) -> Vec<isize> {
    (1..=count)
        .map(|offset| intcode[prog_counter + offset])
        .collect()
}

// Helper function to resolve parameter value based on mode
fn get_parameter_value(intcode: &[isize], param: isize, mode: isize) -> isize {
    match mode {
        0 => intcode[param as usize], // Position mode
        1 => param,                   // Immediate mode
        _ => panic!("Invalid parameter mode: {mode}"),
    }
}
