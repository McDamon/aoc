#[derive(Debug)]
pub enum Opcode {
    Add = 1isize,
    Multiply = 2isize,
    Halt = 99isize,
}

impl TryFrom<isize> for Opcode {
    type Error = String;

    fn try_from(value: isize) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Opcode::Add),
            2 => Ok(Opcode::Multiply),
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

pub fn run_intcode(intcode: &mut [isize], prog_counter: usize) -> &[isize] {
    match Opcode::try_from(intcode[prog_counter]) {
        Ok(Opcode::Add) => {
            calc_add(intcode, prog_counter);
            run_intcode(intcode, prog_counter + 4)
        }
        Ok(Opcode::Multiply) => {
            calc_multiply(intcode, prog_counter);
            run_intcode(intcode, prog_counter + 4)
        }
        Ok(Opcode::Halt) => intcode,
        Err(_) => panic!(),
    }
}

pub fn get_param_pos(intcode: &mut [isize], prog_counter: usize) -> (usize, usize, usize) {
    let param_pos1 = intcode[prog_counter + 1];
    let param_pos2 = intcode[prog_counter + 2];
    let param_pos3 = intcode[prog_counter + 3];
    (
        param_pos1 as usize,
        param_pos2 as usize,
        param_pos3 as usize,
    )
}

pub fn calc_add(intcode: &mut [isize], prog_counter: usize) {
    let param_pos = get_param_pos(intcode, prog_counter);
    intcode[param_pos.2] = intcode[param_pos.0] + intcode[param_pos.1];
}

pub fn calc_multiply(intcode: &mut [isize], prog_counter: usize) {
    let param_pos = get_param_pos(intcode, prog_counter);
    intcode[param_pos.2] = intcode[param_pos.0] * intcode[param_pos.1];
}
