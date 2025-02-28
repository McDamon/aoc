// https://adventofcode.com/2024/day/24

use std::collections::{HashMap, HashSet};

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

    let mut done_gates = HashSet::new();

    let gate_calcs: Vec<_> = input
        .gates
        .iter()
        .map(|gate| {
            let input_wire1_val = input.init_wires.get(&gate.input_wire1).copied();
            let input_wire2_val = input.init_wires.get(&gate.input_wire2).copied();
            let output_wire_val = match (input_wire1_val, input_wire2_val) {
                (Some(wire1), Some(wire2)) => {
                    done_gates.insert(gate.output_wire.clone());
                    Some(get_gate_result(gate, wire1, wire2))
                }
                _ => None,
            };
            GateCalc {
                gate: (*gate).clone(),
                input_wire1_val,
                input_wire2_val,
                output_wire_val,
            }
        })
        .collect();

    loop {
        for gate_calc in &gate_calcs {
            if done_gates.contains(&gate_calc.gate.output_wire) {
                continue;
            }

            let input_wire1_val = match gate_calc.input_wire1_val {
                Some(val) => val,
                None => {
                    if done_gates.contains(&gate_calc.gate.input_wire1)
                        && let Some(&input_wire_1) =
                            input.init_wires.get(&gate_calc.gate.input_wire1)
                    {
                        input_wire_1
                    } else {
                        continue;
                    }
                }
            };

            let input_wire2_val = match gate_calc.input_wire2_val {
                Some(val) => val,
                None => {
                    if done_gates.contains(&gate_calc.gate.input_wire2)
                        && let Some(&input_wire_2) =
                            input.init_wires.get(&gate_calc.gate.input_wire2)
                    {
                        input_wire_2
                    } else {
                        continue;
                    }
                }
            };

            let output_wire_val =
                get_gate_result(&gate_calc.gate, input_wire1_val, input_wire2_val);
            println!(
                "Gate: {:?}, Input1: {:?}, Input2: {:?}, Output: {:?}",
                gate_calc.gate, input_wire1_val, input_wire2_val, output_wire_val
            );
            done_gates.insert(gate_calc.gate.output_wire.clone());
        }

        if done_gates.len() == gate_calcs.len() {
            break;
        }
    }

    for gate_calc in &gate_calcs {
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
        .filter(|gate_calc| gate_calc.gate.output_wire.starts_with('z'))
        .sorted()
        .rev()
        .map(|gate_calc| gate_calc.output_wire_val.unwrap())
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
        assert_eq!(0, get_z_decimal_num("input/2024/day24.txt"));
    }
}
