// https://adventofcode.com/2024/day/24

use std::{collections::HashMap, iter::zip};

use itertools::Itertools;

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

#[derive(Debug, Eq, PartialEq, Clone)]
enum Operation {
    And,
    Or,
    Xor,
}

#[derive(Debug, Clone, Eq, PartialEq)]
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
    let mut gates = vec![];
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
                gates.push(Gate {
                    input_wire1: input1.to_string(),
                    input_wire2: input2.to_string(),
                    op: operation,
                    output_wire: output.to_string(),
                });
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

    println!("Z output vals: {:?}", z_output_vals);

    bin_to_dec(&z_output_vals)
}

pub fn full_adder(bit_num: usize, a: bool, b: bool, c_in: bool, gates: &[Gate]) -> (bool, bool) {
    // sum, c_out
    let a_xor_b = a ^ b;
    let sum = a_xor_b ^ c_in;
    let c_in_and_a_or_b = c_in & a_xor_b;
    let a_and_b = a & b;
    let c_out = c_in_and_a_or_b | a_and_b;

    let input_gate_name_a = format!("x{:02}", bit_num);
    let input_gate_name_b = format!("y{:02}", bit_num);
    let output_gate_name_sum = format!("z{:02}", bit_num);

    // We know that for the a_xor_b gate, the input names are "x_{bit_num}" and "y_{bit_num}", and the output name cannot be "z_{bit_num}"

    // First check whether we have a swapped output
    let all_gates_a_xor_b: Vec<Gate> = gates
        .iter()
        .filter(|gate| {
            (gate.input_wire1 == input_gate_name_a || gate.input_wire1 == input_gate_name_b)
                && (gate.input_wire2 == input_gate_name_b || gate.input_wire2 == input_gate_name_a)
                && gate.op == Operation::Xor
        })
        .cloned()
        .into_iter()
        .collect();

    let mut maybe_gate_a_xor_b: Option<Gate> = None;

    for gate in all_gates_a_xor_b {
        if gate.output_wire == output_gate_name_sum {
            println!("gate.output_wire: {:?}", gate.output_wire);
        } else {
            maybe_gate_a_xor_b = Some(gate.clone());
        }
    }

    // We know that the sum gate will have an output name of "z_{bit_num}"
    let all_gates_sum: Vec<Gate> = gates
        .iter()
        .filter(|gate| gate.output_wire == format!("z{:02}", bit_num))
        .cloned()
        .into_iter()
        .collect();

    let mut maybe_gate_sum: Option<Gate> = None;

    for gate in all_gates_sum {
        if gate.output_wire == output_gate_name_sum {
            maybe_gate_sum = Some(gate.clone());
        } else {
            println!("gate.output_wire: {:?}", gate.output_wire);
        }
    }

    if let Some(gate_sum) = maybe_gate_sum {
        // We know that the c_in_and_a_or_b gate will have an input name of c_in and a_xor_b from the sum gate
        let all_gates_c_in_and_a_or_b: Vec<Gate> = gates
            .iter()
            .filter(|gate| {
                (gate.input_wire1 == gate_sum.input_wire1
                    || gate.input_wire1 == gate_sum.input_wire2)
                    && (gate.input_wire2 == gate_sum.input_wire1
                        || gate.input_wire2 == gate_sum.input_wire2)
                    && gate.op == Operation::And
            })
            .cloned()
            .into_iter()
            .collect();

        let maybe_gate_c_in_and_a_or_b: Option<Gate> = None;

        // We know that the a_and_b gate will have an input name of "x_{bit_num}" and "y_{bit_num}"
        let all_gates_a_and_b: Vec<Gate> = gates
            .iter()
            .find(|gate| {
                (gate.input_wire1 == input_gate_name_a || gate.input_wire1 == input_gate_name_b)
                    && (gate.input_wire2 == input_gate_name_a
                        || gate.input_wire2 == input_gate_name_b)
            })
            .cloned()
            .into_iter()
            .collect();

        let mut maybe_gate_a_and_b: Option<Gate> = None;

        if let Some(gate_a_and_b) = maybe_gate_a_and_b {
            if let Some(gate_c_in_and_a_or_b) = maybe_gate_c_in_and_a_or_b {
                println!("c_in_and_a_or_b: {:?}", c_in_and_a_or_b);

                // We know that the c_out gate will have an input name of c_in_and_a_or_b and a_and_b from the sum gate
                let all_gates_c_out: Vec<Gate> = gates
                    .iter()
                    .filter(|gate| {
                        (gate.input_wire1 == gate_c_in_and_a_or_b.output_wire
                            || gate.input_wire1 == gate_a_and_b.output_wire)
                            && (gate.input_wire2 == gate_c_in_and_a_or_b.output_wire
                                || gate.input_wire2 == gate_a_and_b.output_wire)
                            && gate.op == Operation::Or
                    })
                    .cloned()
                    .into_iter()
                    .collect();
            }
        }
    }

    (sum, c_out)
}

pub fn ripple_adder(a_bits: &[bool], b_bits: &[bool], gates: &[Gate]) -> (Vec<bool>, Vec<bool>) {
    // sums, c_outs
    let mut sums = vec![];
    let mut c_outs = vec![false];

    for (bit_num, (a, b)) in zip(a_bits, b_bits).enumerate() {
        if let Some(c_out) = c_outs.last() {
            let (sum, c_out) = full_adder(bit_num, *a, *b, *c_out, gates);
            sums.push(sum);
            c_outs.push(c_out);
        }
    }

    if let Some(c_out) = c_outs.last() {
        sums.push(*c_out);
    }

    (sums, c_outs)
}

pub fn get_swapped_wires(input_file: &str) -> &str {
    let input = parse_input(input_file);

    let num_bits = input.init_wires.len() / 2;
    println!("num_bits: {:?}", num_bits);

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

    println!("a_bits: {:?}", a_bits);
    println!("b_bits: {:?}", b_bits);

    let (sums, c_outs) = ripple_adder(&a_bits, &b_bits, &input.gates);

    println!("sums: {:?}", sums);
    println!("c_outs: {:?}", c_outs);

    ""
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
        let gates = vec![];
        assert_eq!((false, false), full_adder(0, false, false, false, &gates));
        assert_eq!((true, false), full_adder(0, false, false, true, &gates));
        assert_eq!((true, false), full_adder(0, false, true, false, &gates));
        assert_eq!((false, true), full_adder(0, false, true, true, &gates));
        assert_eq!((true, false), full_adder(0, true, false, false, &gates));
        assert_eq!((false, true), full_adder(0, true, false, true, &gates));
        assert_eq!((false, true), full_adder(0, true, true, false, &gates));
        assert_eq!((true, true), full_adder(0, true, true, true, &gates));
    }

    #[test]
    fn test_ripple_adder() {
        let a_bits = vec![true, true, false, true];
        let b_bits = vec![true, false, true, true];
        let sums = vec![false, false, false, true, true];
        let c_outs = vec![false, true, true, true, true];
        let gates = vec![];
        assert_eq!((sums, c_outs), ripple_adder(&a_bits, &b_bits, &gates));
    }

    #[test]
    fn test_get_swapped_wires() {
        assert_eq!("", get_swapped_wires("input/2024/day24.txt"));
    }
}
