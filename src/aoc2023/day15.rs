// https://adventofcode.com/2023/day/15

use std::collections::HashMap;

use csv::ReaderBuilder;
use regex::Regex;

use crate::utils::get_lines;

#[derive(Debug)]
struct Input {
    steps: Vec<String>,
}

#[derive(Debug)]
enum Operation {
    Dash,
    Equals,
}

#[derive(Debug)]
struct Step {
    label: String,
    operation: Operation,
    focal: Option<u32>,
}

#[derive(Debug)]
struct Slot {
    label: String,
    focal: u32,
}

fn parse_input(input_file: &str) -> Input {
    let lines = get_lines(input_file);

    let mut steps: Vec<String> = vec![];
    for line in lines.into_iter() {
        let mut reader = ReaderBuilder::new()
            .has_headers(false)
            .from_reader(line.as_bytes());
        for maybe_records in reader.records() {
            match maybe_records {
                Ok(records) => {
                    for record in records.into_iter() {
                        steps.push(record.to_owned());
                    }
                }
                Err(_) => panic!("invalid step"),
            }
        }
    }

    Input { steps }
}

fn get_sum_steps(input_file: &str) -> u32 {
    let input = parse_input(input_file);

    let mut sum_steps = 0;

    for step in input.steps {
        sum_steps += compute_hash(&step);
    }
    sum_steps
}

fn compute_hash(step: &str) -> u32 {
    let mut current_value: u32 = 0;

    for char in step.chars() {
        if char != '\n' {
            let ascii_code = char as u8;
            current_value += ascii_code as u32;
            current_value *= 17;
            current_value %= 256;
        }
    }

    current_value
}

fn get_focusing_power(input_file: &str) -> u32 {
    lazy_static! {
        static ref RE_HASHMAP: Regex =
            Regex::new(r"(?P<label>[a-z]+)(?P<operation>-|={1})(?P<focal>\d*)").unwrap();
    }
    let input = parse_input(input_file);

    let mut steps: Vec<Step> = vec![];
    for step in input.steps {
        let maybe_caps_hashmap = RE_HASHMAP.captures(&step);
        if let Some(caps_hashmap) = maybe_caps_hashmap {
            let label_str = caps_hashmap["label"].to_string();
            let operation_str = caps_hashmap["operation"].to_string();
            let focal_str = caps_hashmap["focal"].to_string();

            let operation = match operation_str.as_str() {
                "-" => Some(Operation::Dash),
                "=" => Some(Operation::Equals),
                _ => None,
            }
            .unwrap();

            let focal = match operation {
                Operation::Equals => Some(focal_str.parse().unwrap()),
                Operation::Dash => None,
            };

            let step = Step {
                label: label_str,
                operation,
                focal,
            };
            steps.push(step);
        }
    }

    let mut boxes: HashMap<u32, Vec<Slot>> = HashMap::new();

    // Initialise boxes first
    for step in &steps {
        let box_index = compute_hash(&step.label);
        boxes.insert(box_index, vec![]);
    }

    for step in steps {
        let box_index = compute_hash(&step.label);
        match step.operation {
            Operation::Dash => {
                boxes.entry(box_index).and_modify(|slots| {
                    let maybe_slot_index = slots.iter().position(|slot| slot.label == step.label);
                    if let Some(slot_index) = maybe_slot_index {
                        slots.remove(slot_index);
                    }
                });
            }
            Operation::Equals => {
                boxes.entry(box_index).and_modify(|slots| {
                    let maybe_slot_index = slots.iter().position(|slot| slot.label == step.label);
                    if let Some(slot_index) = maybe_slot_index {
                        slots.remove(slot_index);
                        slots.insert(
                            slot_index,
                            Slot {
                                label: step.label,
                                focal: step.focal.unwrap(),
                            },
                        );
                    } else {
                        slots.push(Slot {
                            label: step.label,
                            focal: step.focal.unwrap(),
                        });
                    }
                });
            }
        }
    }

    let mut focusing_power = 0;

    for box_index in boxes.keys() {
        let maybe_slots = boxes.get(box_index);
        if let Some(slots) = maybe_slots {
            for (pos, slot) in slots.iter().enumerate() {
                focusing_power += (*box_index + 1) * (pos as u32 + 1) * slot.focal;
            }
        }
    }

    focusing_power
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_hash_test01() {
        assert_eq!(30, compute_hash("rn=1"));
    }

    #[test]
    fn test_compute_hash_test02() {
        assert_eq!(253, compute_hash("cm-"));
    }

    #[test]
    fn test_compute_hash_test03() {
        assert_eq!(97, compute_hash("qp=3"));
    }

    #[test]
    fn test_compute_hash_test04() {
        assert_eq!(47, compute_hash("cm=2"));
    }

    #[test]
    fn test_compute_hash_test05() {
        assert_eq!(14, compute_hash("qp-"));
    }

    #[test]
    fn test_compute_hash_test06() {
        assert_eq!(180, compute_hash("pc=4"));
    }

    #[test]
    fn test_compute_hash_test07() {
        assert_eq!(9, compute_hash("ot=9"));
    }

    #[test]
    fn test_compute_hash_test08() {
        assert_eq!(197, compute_hash("ab=5"));
    }

    #[test]
    fn test_compute_hash_test09() {
        assert_eq!(48, compute_hash("pc-"));
    }

    #[test]
    fn test_compute_hash_test10() {
        assert_eq!(214, compute_hash("pc=6"));
    }

    #[test]
    fn test_compute_hash_test11() {
        assert_eq!(231, compute_hash("ot=7"));
    }

    #[test]
    fn test_compute_hash_test12() {
        assert_eq!(52, compute_hash("HASH"));
    }

    #[test]
    fn test_get_num_steps_test01() {
        assert_eq!(1320, get_sum_steps("input/2023/day15_test01.txt"));
    }

    #[test]
    fn test_get_sum_steps() {
        assert_eq!(507666, get_sum_steps("input/2023/day15.txt"));
    }

    #[test]
    fn test_get_focusing_power_test01() {
        assert_eq!(145, get_focusing_power("input/2023/day15_test01.txt"));
    }

    #[test]
    fn test_get_focusing_power() {
        assert_eq!(233537, get_focusing_power("input/2023/day15.txt"));
    }
}
