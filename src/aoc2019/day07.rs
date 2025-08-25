// https://adventofcode.com/2019/day/7

use itertools::Itertools;

use crate::intcode::{Opcode, parse_intcode_input, run_intcode};

pub fn run_amplifier(
    intcode: &mut [isize],
    prog_counter: &mut usize,
    maybe_phase: Option<isize>,
    maybe_input: Option<isize>,
) -> isize {
    let mut outputs = vec![];
    let input = maybe_input.unwrap_or_default();
    let mut inputs = vec![input];
    if let Some(phase) = maybe_phase {
        inputs.push(phase);
    }

    println!("inputs: {:?}", inputs);

    run_intcode(intcode, prog_counter, &mut inputs, &mut outputs);

    println!("outputs: {:?}", outputs);

    *outputs.first().unwrap()
}

pub fn run_amplifiers_part_one(intcode: &[isize], phases: &[isize]) -> isize {
    let mut intcode_a = intcode.to_vec();
    let mut intcode_b = intcode.to_vec();
    let mut intcode_c = intcode.to_vec();
    let mut intcode_d = intcode.to_vec();
    let mut intcode_e = intcode.to_vec();

    let output_a = run_amplifier(&mut intcode_a, &mut 0, Some(phases[0]), None);
    println!("output_a: {}", output_a);
    let output_b = run_amplifier(&mut intcode_b, &mut 0, Some(phases[1]), Some(output_a));
    println!("output_b: {}", output_b);
    let output_c = run_amplifier(&mut intcode_c, &mut 0, Some(phases[2]), Some(output_b));
    println!("output_c: {}", output_c);
    let output_d = run_amplifier(&mut intcode_d, &mut 0, Some(phases[3]), Some(output_c));
    println!("output_d: {}", output_d);
    let output_e = run_amplifier(&mut intcode_e, &mut 0, Some(phases[4]), Some(output_d));
    println!("output_e: {}", output_e);
    output_e
}

pub fn get_highest_signal_part_one(input_file: &str) -> isize {
    let input_intcode = parse_intcode_input(input_file);

    let phase_vals = [0, 1, 2, 3, 4];

    let mut signals = vec![];
    for perm in phase_vals.iter().permutations(phase_vals.len()).unique() {
        println!("Running amplifier sequence: {:?}", perm);
        let signal = run_amplifiers_part_one(
            &input_intcode,
            &perm.into_iter().copied().collect::<Vec<isize>>(),
        );
        signals.push(signal);
    }

    *signals.iter().max().unwrap()
}

pub fn run_amplifiers_part_two(intcode: &[isize], phases: &[isize]) -> isize {
    let mut output = 0;
    let mut first_run = true;

    let mut intcode_a = intcode.to_vec();
    let mut intcode_b = intcode.to_vec();
    let mut intcode_c = intcode.to_vec();
    let mut intcode_d = intcode.to_vec();
    let mut intcode_e = intcode.to_vec();

    let mut prog_counter_a = 0;
    let mut prog_counter_b = 0;
    let mut prog_counter_c = 0;
    let mut prog_counter_d = 0;
    let mut prog_counter_e = 0;
    loop {
        println!("Run amplifier a at prog counter: {}", prog_counter_a);
        output = run_amplifier(
            &mut intcode_a,
            &mut prog_counter_a,
            if first_run { Some(phases[0]) } else { None },
            if first_run { None } else { Some(output) },
        );
        println!("output_a: {}", output);
        let last_opcode_a = intcode[prog_counter_a];

        println!("Run amplifier b at prog counter: {}", prog_counter_b);
        output = run_amplifier(
            &mut intcode_b,
            &mut prog_counter_b,
            if first_run { Some(phases[1]) } else { None },
            Some(output),
        );
        println!("output_b: {}", output);
        let last_opcode_b = intcode[prog_counter_b];

        println!("Run amplifier c at prog counter: {}", prog_counter_c);
        output = run_amplifier(
            &mut intcode_c,
            &mut prog_counter_c,
            if first_run { Some(phases[2]) } else { None },
            Some(output),
        );
        println!("output_c: {}", output);
        let last_opcode_c = intcode[prog_counter_c];

        println!("Run amplifier d at prog counter: {}", prog_counter_d);
        output = run_amplifier(
            &mut intcode_d,
            &mut prog_counter_d,
            if first_run { Some(phases[3]) } else { None },
            Some(output),
        );
        println!("output_d: {}", output);
        let last_opcode_d = intcode[prog_counter_d];

        println!("Run amplifier e at prog counter: {}", prog_counter_e);
        output = run_amplifier(
            &mut intcode_e,
            &mut prog_counter_e,
            if first_run { Some(phases[4]) } else { None },
            Some(output),
        );
        println!("output_e: {}", output);
        let last_opcode_e = intcode[prog_counter_e];

        if last_opcode_a == Opcode::Halt as isize
            || last_opcode_b == Opcode::Halt as isize
            || last_opcode_c == Opcode::Halt as isize
            || last_opcode_d == Opcode::Halt as isize
            || last_opcode_e == Opcode::Halt as isize
        {
            break;
        };
        first_run = false;
    }
    output
}

pub fn get_highest_signal_part_two(input_file: &str) -> isize {
    let input_intcode = parse_intcode_input(input_file);

    let phase_vals = [9, 8, 7, 6, 5];

    let mut signals = vec![];

    for perm in phase_vals.iter().permutations(phase_vals.len()).unique() {
        println!("Running amplifier sequence: {:?}", perm);
        let signal = run_amplifiers_part_two(
            &input_intcode,
            &perm.into_iter().copied().collect::<Vec<isize>>(),
        );
        signals.push(signal);
    }

    *signals.iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_highest_signal_test01() {
        assert_eq!(
            43210,
            get_highest_signal_part_one("input/2019/day07_test01.txt")
        );
    }

    #[test]
    fn test_get_highest_signal_test02() {
        assert_eq!(
            54321,
            get_highest_signal_part_one("input/2019/day07_test02.txt")
        );
    }

    #[test]
    fn test_get_highest_signal_test03() {
        assert_eq!(
            65210,
            get_highest_signal_part_one("input/2019/day07_test03.txt")
        );
    }

    #[test]
    fn test_get_highest_signal_part_one() {
        assert_eq!(567045, get_highest_signal_part_one("input/2019/day07.txt"));
    }

    #[test]
    fn test_get_highest_signal_test04() {
        assert_eq!(
            139629729,
            get_highest_signal_part_two("input/2019/day07_test04.txt")
        );
    }

    #[test]
    fn test_get_highest_signal_test05() {
        assert_eq!(
            18216,
            get_highest_signal_part_two("input/2019/day07_test05.txt")
        );
    }

    #[test]
    fn test_get_highest_signal_part_two() {
        assert_eq!(39016654, get_highest_signal_part_two("input/2019/day07.txt"));
    }
}
