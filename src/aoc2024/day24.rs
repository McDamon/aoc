// https://adventofcode.com/2024/day/24

use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    iter::zip,
    vec,
};

use crate::utils::get_lines;

struct Input {
    init_wires: HashMap<String, bool>,
    gates: Vec<Gate>,
}

impl Ord for GateCalc {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.gate.output_wire.cmp(&other.gate.output_wire)
    }
}

impl PartialOrd for GateCalc {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
enum Operation {
    And,
    Or,
    Xor,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Gate {
    input_wire1: String,
    input_wire2: String,
    op: Operation,
    output_wire: String,
}

#[derive(Debug, Eq, PartialEq)]
pub struct GateCalc {
    gate: Gate,
    input_wire1_val: Option<bool>,
    input_wire2_val: Option<bool>,
    output_wire_val: Option<bool>,
}

fn parse_input(input_file: &str) -> Input {
    let lines = get_lines(input_file);

    let mut init_wires = HashMap::new();
    let mut gates = Vec::new();
    let mut parsing_gates = false;

    for line in lines {
        if line.trim().is_empty() {
            parsing_gates = true;
            continue;
        }

        if parsing_gates {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if let [input1, op, input2, _, output] = &parts[..] {
                let operation = match *op {
                    "AND" => Operation::And,
                    "OR" => Operation::Or,
                    "XOR" => Operation::Xor,
                    _ => panic!("Unknown operation"),
                };
                let gate = Gate {
                    input_wire1: input1.to_string(),
                    input_wire2: input2.to_string(),
                    op: operation,
                    output_wire: output.to_string(),
                };
                if !gates.contains(&gate) {
                    gates.push(gate.clone());
                } else {
                    panic!("Duplicate gate found: {gate:?}");
                }
            }
        } else {
            let parts: Vec<&str> = line.split(':').collect();
            if let [wire, value] = &parts[..] {
                init_wires.insert(
                    wire.trim().to_string(),
                    value.trim().parse::<usize>().unwrap() != 0,
                );
            }
        }
    }

    Input { init_wires, gates }
}

fn get_gate_result(gate: &Gate, wire1: bool, wire2: bool) -> bool {
    match gate.op {
        Operation::And => wire1 & wire2,
        Operation::Or => wire1 | wire2,
        Operation::Xor => wire1 ^ wire2,
    }
}

fn bin_to_dec(binary_digits: &[bool]) -> usize {
    binary_digits
        .iter()
        .enumerate()
        .fold(0, |acc, (i, &digit)| {
            acc + (digit as usize * 2_usize.pow(i as u32))
        })
}

fn get_gate_calcs(input: &Input) -> HashMap<String, GateCalc> {
    let mut gate_calcs: HashMap<String, GateCalc> = input
        .gates
        .iter()
        .map(|gate| {
            let input_wire1_val = input.init_wires.get(&gate.input_wire1).copied();
            let input_wire2_val = input.init_wires.get(&gate.input_wire2).copied();
            let output_wire_val = match (input_wire1_val, input_wire2_val) {
                (Some(wire1), Some(wire2)) => Some(get_gate_result(gate, wire1, wire2)),
                _ => None,
            };
            (
                gate.output_wire.clone(),
                GateCalc {
                    gate: (*gate).clone(),
                    input_wire1_val,
                    input_wire2_val,
                    output_wire_val,
                },
            )
        })
        .collect();

    println!("POST INIT GATE CALCS");
    for (_output_wire, gate_calc) in gate_calcs.iter_mut() {
        println!(
            "Gate: {:?}, Input1: {:?}, Input2: {:?}, Output: {:?}",
            gate_calc.gate,
            gate_calc.input_wire1_val,
            gate_calc.input_wire2_val,
            gate_calc.output_wire_val
        );
    }

    loop {
        let mut updates = vec![];
        for gate_calc in gate_calcs.values() {
            if gate_calc.output_wire_val.is_none() {
                let input_wire1_val = gate_calc.input_wire1_val.or_else(|| {
                    gate_calcs
                        .get(&gate_calc.gate.input_wire1)
                        .and_then(|gate_calc| gate_calc.output_wire_val)
                });
                let input_wire2_val = gate_calc.input_wire2_val.or_else(|| {
                    gate_calcs
                        .get(&gate_calc.gate.input_wire2)
                        .and_then(|gate_calc| gate_calc.output_wire_val)
                });
                let output_wire_val = match (input_wire1_val, input_wire2_val) {
                    (Some(wire1), Some(wire2)) => {
                        Some(get_gate_result(&gate_calc.gate, wire1, wire2))
                    }
                    _ => None,
                };
                updates.push((gate_calc.gate.output_wire.clone(), output_wire_val));
            }
        }

        if updates.is_empty() {
            break;
        }

        for (output_wire, output_wire_val) in updates {
            gate_calcs.get_mut(&output_wire).unwrap().output_wire_val = output_wire_val;
        }
    }

    gate_calcs
}

fn get_z_output_vals(gate_calcs: &HashMap<String, GateCalc>) -> Vec<bool> {
    gate_calcs
        .iter()
        .filter(|(_output_wire, gate_calc)| gate_calc.gate.output_wire.starts_with('z'))
        .sorted()
        .map(|(_output_wire, gate_calc)| gate_calc.output_wire_val.unwrap())
        .collect::<Vec<_>>()
}

pub fn get_z_decimal_num(input_file: &str) -> usize {
    let input = parse_input(input_file);

    let gate_calcs = get_gate_calcs(&input);

    println!("POST UPDATE GATE CALCS");
    for gate_calc in gate_calcs.values() {
        println!(
            "Gate: {:?}, Input1: {:?}, Input2: {:?}, Output: {:?}",
            gate_calc.gate,
            gate_calc.input_wire1_val,
            gate_calc.input_wire2_val,
            gate_calc.output_wire_val
        );
    }

    let z_output_vals = get_z_output_vals(&gate_calcs);

    println!("Z output vals: {z_output_vals:?}");

    bin_to_dec(&z_output_vals)
}

pub fn full_adder(a: bool, b: bool, c_in: bool) -> (bool, bool) {
    // sum, c_out
    let a_xor_b = a ^ b;
    let sum = a_xor_b ^ c_in;
    let c_in_and_a_or_b = c_in & a_xor_b;
    let a_and_b = a & b;
    let c_out = c_in_and_a_or_b | a_and_b;

    (sum, c_out)
}

pub fn ripple_adder(a_bits: &[bool], b_bits: &[bool]) -> (Vec<bool>, Vec<bool>) {
    // sums, c_outs
    let mut sums = vec![];
    let mut c_outs = vec![false];

    for (a, b) in zip(a_bits, b_bits) {
        if let Some(c_out) = c_outs.last() {
            let (sum, c_out) = full_adder(*a, *b, *c_out);
            sums.push(sum);
            c_outs.push(c_out);
        }
    }

    if let Some(c_out) = c_outs.last() {
        sums.push(*c_out);
    }

    (sums, c_outs)
}

pub fn get_swapped_wires(input_file: &str) -> String {
    let input = parse_input(input_file);

    let num_bits = input.init_wires.len() / 2;
    println!("num_bits: {num_bits:?}");

    let mut a_bits = vec![];
    let mut b_bits = vec![];

    for bit_num in 0..num_bits {
        let bit_num_str = bit_num.to_string();
        let x_key: String = "x".to_owned() + &bit_num_str;
        let y_key = "y".to_owned() + &bit_num_str;
        if let Some(x_val) = input.init_wires.get(&x_key)
            && let Some(y_val) = input.init_wires.get(&y_key)
        {
            a_bits.push(*x_val);
            b_bits.push(*y_val);
        }
    }

    println!("a_bits: {a_bits:?}");
    println!("b_bits: {b_bits:?}");

    let (sums, c_outs) = ripple_adder(&a_bits, &b_bits);

    println!("sums: {sums:?}");
    println!("c_outs: {c_outs:?}");

    swap_wires(&input.gates.clone()).to_string()
}

fn swap_wires(gates: &[Gate]) -> String {
    let mut initial_invalid_gates: HashSet<Gate> = HashSet::new();

    match has_swapped_wires(gates) {
        Ok(_) => (),
        Err(new_invalid_gates) => {
            initial_invalid_gates.extend(new_invalid_gates);
        }
    }

    for initial_invalid_gate in initial_invalid_gates.clone() {
        println!("Initial invalid gate: {initial_invalid_gate:?}");
    }

    let output_wires = initial_invalid_gates
        .iter()
        .map(|gate| gate.output_wire.clone())
        .sorted_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()))
        .collect::<Vec<_>>()
        .join(","); // Join the sorted output wires with commas

    println!("Output wires: {output_wires:?}");

    output_wires // Return the comma-separated string
}

fn has_swapped_wires(gates: &[Gate]) -> Result<Vec<Gate>, Vec<Gate>> {
    let mut valid_gates: Vec<Gate> = Vec::new();
    let mut invalid_gates: Vec<Gate> = Vec::new();

    // Helped by https://www.reddit.com/r/adventofcode/comments/1hla5ql/2024_day_24_part_2_a_guide_on_the_idea_behind_the/
    let wrong1: Vec<_> = gates
        .iter()
        .filter(|gate| {
            gate.output_wire.contains('z') && gate.op != Operation::Xor && gate.output_wire != "z45"
        })
        .cloned()
        .collect();

    let wrong2: Vec<_> = gates
        .iter()
        .filter(|gate| {
            !gate.output_wire.contains('z')
                && !gate.input_wire1.contains('x')
                && !gate.input_wire2.contains('x')
                && !gate.input_wire1.contains('y')
                && !gate.input_wire2.contains('y')
                && gate.op == Operation::Xor
        })
        .cloned()
        .collect();

    let wrong3: Vec<_> = gates
        .iter()
        .filter(|gate| {
            (gate.input_wire1.contains('x')
                || gate.input_wire2.contains('x')
                || gate.input_wire1.contains('y')
                || gate.input_wire2.contains('y'))
                && gate.op == Operation::Xor
                && gates.iter().all(|other_gate| {
                    (other_gate.input_wire1 != gate.output_wire
                        && other_gate.input_wire2 != gate.output_wire)
                        || other_gate.op != Operation::Xor
                })
        })
        .filter(|gate| !gate.input_wire1.contains("00"))
        .cloned()
        .collect();

    let wrong4: Vec<_> = gates
        .iter()
        .filter(|gate| {
            (gate.input_wire1.contains('x')
                || gate.input_wire2.contains('x')
                || gate.input_wire1.contains('y')
                || gate.input_wire2.contains('y'))
                && gate.op == Operation::And
                && gates.iter().all(|other_gate| {
                    (other_gate.input_wire1 != gate.output_wire
                        && other_gate.input_wire2 != gate.output_wire)
                        || other_gate.op != Operation::Or
                })
        })
        .filter(|gate| !gate.input_wire1.contains("00"))
        .cloned()
        .collect();

    // Combine all invalid gates
    invalid_gates.extend(wrong1);
    invalid_gates.extend(wrong2);
    invalid_gates.extend(wrong3);
    invalid_gates.extend(wrong4);

    // Identify valid gates
    valid_gates.extend(
        gates
            .iter()
            .filter(|gate| !invalid_gates.contains(gate))
            .cloned(),
    );

    if invalid_gates.is_empty() {
        Ok(valid_gates)
    } else {
        Err(invalid_gates)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_z_decimal_num_test01() {
        assert_eq!(4, get_z_decimal_num("input/2024/day24_test01.txt"));
    }

    #[test]
    fn test_get_z_decimal_num_test02() {
        assert_eq!(2024, get_z_decimal_num("input/2024/day24_test02.txt"));
    }

    #[test]
    fn test_get_z_decimal_num() {
        assert_eq!(65635066541798, get_z_decimal_num("input/2024/day24.txt"));
    }

    #[test]
    fn test_full_adder() {
        assert_eq!((false, false), full_adder(false, false, false));
        assert_eq!((true, false), full_adder(false, false, true));
        assert_eq!((true, false), full_adder(false, true, false));
        assert_eq!((false, true), full_adder(false, true, true));
        assert_eq!((true, false), full_adder(true, false, false));
        assert_eq!((false, true), full_adder(true, false, true));
        assert_eq!((false, true), full_adder(true, true, false));
        assert_eq!((true, true), full_adder(true, true, true));
    }

    #[test]
    fn test_ripple_adder() {
        let a_bits = vec![true, true, false, true];
        let b_bits = vec![true, false, true, true];
        let sums = vec![false, false, false, true, true];
        let c_outs = vec![false, true, true, true, true];
        assert_eq!((sums, c_outs), ripple_adder(&a_bits, &b_bits));
    }

    #[test]
    fn test_get_swapped_wires() {
        assert_eq!(
            "dgr,dtv,fgc,mtj,vvm,z12,z29,z37",
            get_swapped_wires("input/2024/day24.txt")
        );
    }
}
