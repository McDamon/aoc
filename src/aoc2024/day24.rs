// https://adventofcode.com/2024/day/24

use std::collections::HashMap;

use itertools::Itertools;

use crate::utils::get_lines;

struct Input {
    init_wires: HashMap<String, usize>,
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
struct Gate {
    input_wire1: String,
    input_wire2: String,
    op: Operation,
    output_wire: String,
}

#[derive(Debug, Eq, PartialEq)]
struct GateCalc {
    gate: Gate,
    input_wire1_val: Option<usize>,
    input_wire2_val: Option<usize>,
    output_wire_val: Option<usize>,
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
                init_wires.insert(wire.trim().to_string(), value.trim().parse().unwrap());
            }
        }
    }

    Input { init_wires, gates }
}

fn get_gate_result(gate: &Gate, wire1: usize, wire2: usize) -> usize {
    match gate.op {
        Operation::And => wire1 & wire2,
        Operation::Or => wire1 | wire2,
        Operation::Xor => wire1 ^ wire2,
    }
}

fn bin_to_dec(binary_digits: &[usize]) -> usize {
    binary_digits
        .iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (i, &digit)| acc + (digit * 2_usize.pow(i as u32)))
}

pub fn get_z_decimal_num(input_file: &str) -> usize {
    let input = parse_input(input_file);

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

    let z_output_vals = gate_calcs
        .iter()
        .filter(|(_output_wire, gate_calc)| gate_calc.gate.output_wire.starts_with('z'))
        .sorted()
        .rev()
        .map(|(_output_wire, gate_calc)| gate_calc.output_wire_val.unwrap())
        .collect::<Vec<_>>();

    println!("Z output vals: {:?}", z_output_vals);

    bin_to_dec(&z_output_vals)
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
    fn test_get_z_decimal_num_() {
        assert_eq!(65635066541798, get_z_decimal_num("input/2024/day24.txt"));
    }
}
