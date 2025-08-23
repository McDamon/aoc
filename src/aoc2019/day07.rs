// https://adventofcode.com/2019/day/7

use itertools::Itertools;

use crate::intcode::{parse_intcode_input, run_intcode};

pub fn run_amplifier(intcode: &[isize], phase: isize, maybe_input: Option<isize>) -> isize {
    let mut prog_counter = 0;
    let mut outputs = vec![];
    let mut intcode_copy = intcode.to_vec();
    let input = maybe_input.unwrap_or_default();
    let mut inputs = vec![input, phase];
    run_intcode(
        &mut intcode_copy,
        &mut prog_counter,
        &mut inputs,
        &mut outputs,
    );

    println!("outputs: {:?}", outputs);

    *outputs.first().unwrap()
}

pub fn run_amplifiers(intcode: &[isize], phases: &[isize]) -> isize {
    let output_a = run_amplifier(intcode, phases[0], None);
    println!("output_a: {}", output_a);
    let output_b = run_amplifier(intcode, phases[1], Some(output_a));
    println!("output_b: {}", output_b);
    let output_c = run_amplifier(intcode, phases[2], Some(output_b));
    println!("output_c: {}", output_c);
    let output_d = run_amplifier(intcode, phases[3], Some(output_c));
    println!("output_d: {}", output_d);
    let output_e = run_amplifier(intcode, phases[4], Some(output_d));
    println!("output_e: {}", output_e);
    output_e
}

pub fn get_highest_signal(input_file: &str) -> isize {
    let input_intcode = parse_intcode_input(input_file);

    let phase_vals = [0, 1, 2, 3, 4];

    let mut signals = vec![];
    for perm in phase_vals.iter().permutations(phase_vals.len()).unique() {
        println!("Running amplifier sequence: {:?}", perm);
        let signal = run_amplifiers(
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
        assert_eq!(43210, get_highest_signal("input/2019/day07_test01.txt"));
    }

    #[test]
    fn test_get_highest_signal_test02() {
        assert_eq!(54321, get_highest_signal("input/2019/day07_test02.txt"));
    }

    #[test]
    fn test_get_highest_signal_test03() {
        assert_eq!(65210, get_highest_signal("input/2019/day07_test03.txt"));
    }

    #[test]
    fn test_get_highest_signal() {
        assert_eq!(567045, get_highest_signal("input/2019/day07.txt"));
    }
}
