// https://adventofcode.com/2023/day/12

use std::{collections::HashMap, str::FromStr};

use crate::utils::get_lines;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum SpringCondition {
    Operational,
    Broken,
    Unknown,
}

#[derive(Debug)]
pub struct ConditionRecord {
    pub spring_conditions: Vec<SpringCondition>,
    pub contiguous_groups: Vec<usize>,
}

#[derive(Debug)]
pub struct Input {
    pub condition_records: Vec<ConditionRecord>,
}

pub fn parse_input(input_file: &str) -> Input {
    let lines = get_lines(input_file);

    let condition_records = parse_condition_records(lines);
    Input { condition_records }
}

fn parse_condition_records(lines: Vec<String>) -> Vec<ConditionRecord> {
    let mut condition_records = vec![];
    for line in lines {
        let maybe_condition_record = line.parse::<ConditionRecord>();
        match maybe_condition_record {
            Ok(condition_record) => condition_records.push(condition_record),
            Err(_) => todo!(),
        }
    }
    condition_records
}

impl TryFrom<char> for SpringCondition {
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Self::Operational),
            '#' => Ok(Self::Broken),
            '?' => Ok(Self::Unknown),
            _ => Err(()),
        }
    }

    type Error = ();
}

impl FromStr for ConditionRecord {
    type Err = ();

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        if let Some((springs_str, groups_str)) = line.split_once(' ') {
            let mut spring_conditions = Vec::new();
            for spring in springs_str.chars() {
                let spring = spring.try_into()?;
                spring_conditions.push(spring);
            }

            let mut contiguous_groups = Vec::new();
            for group in groups_str.split(',') {
                let group = group.parse().map_err(|_| ())?;
                contiguous_groups.push(group);
            }

            Ok(ConditionRecord {
                spring_conditions,
                contiguous_groups,
            })
        } else {
            Err(())
        }
    }
}

pub fn sum_spring_arrangements(input_file: &str, unfold: bool) -> u64 {
    let mut spring_arrangements: u64 = 0;
    let input = parse_input(input_file);
    for condition_record in input.condition_records {
        if unfold {
            spring_arrangements += condition_record
                .unfold()
                .count_spring_arrangements(&mut HashMap::new(), (0, 0));
        } else {
            spring_arrangements +=
                condition_record.count_spring_arrangements(&mut HashMap::new(), (0, 0));
        }
    }
    spring_arrangements
}

impl ConditionRecord {
    // Inspired/plagiarised from https://github.com/andypymont/advent2023-rust/blob/main/src/bin/12.rs
    fn count_spring_arrangements(
        &self,
        cache: &mut HashMap<(usize, usize), u64>,
        (spring_index, group_index): (usize, usize),
    ) -> u64 {
        if let Some(cached_arrangement) = cache.get(&(spring_index, group_index)) {
            return *cached_arrangement;
        }

        let consume = self
            .contiguous_groups
            .get(group_index)
            .map_or(0, |current_group_len| {
                //println!("current_group_len {:?}", current_group_len);
                // Does current group fit within remaining springs?
                if (spring_index + current_group_len) > self.spring_conditions.len() {
                    return 0;
                }
                // Does group contain operational springs?
                if (0..*current_group_len).any(|pos| {
                    self.spring_conditions.get(spring_index + pos)
                        == Some(&SpringCondition::Operational)
                }) {
                    return 0;
                }
                // Is next spring after group a broken spring?
                if self.spring_conditions.get(spring_index + current_group_len)
                    == Some(&SpringCondition::Broken)
                {
                    return 0;
                }

                // We can consume the group
                self.count_spring_arrangements(
                    cache,
                    (spring_index + current_group_len + 1, group_index + 1),
                )
            });

        let skip = match self.spring_conditions.get(spring_index) {
            None => u64::from(group_index >= self.contiguous_groups.len()),
            Some(SpringCondition::Broken) => 0,
            Some(_) => self.count_spring_arrangements(cache, (spring_index + 1, group_index)),
        };

        let spring_arrangements = consume + skip;

        cache.insert((spring_index, group_index), spring_arrangements);

        spring_arrangements
    }

    fn unfold(&self) -> Self {
        let mut spring_conditions = Vec::new();
        let mut contiguous_groups = Vec::new();

        for repeat in 1..=5 {
            spring_conditions.extend(&self.spring_conditions);
            if repeat != 5 {
                spring_conditions.push(SpringCondition::Unknown);
            }
            contiguous_groups.extend(&self.contiguous_groups);
        }

        Self {
            spring_conditions,
            contiguous_groups,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_spring_arrangements_test01() {
        assert_eq!(21, sum_spring_arrangements("input/2023/day12_test01.txt", false));
    }

    #[test]
    fn test_sum_spring_arrangements_test02() {
        assert_eq!(1, sum_spring_arrangements("input/2023/day12_test02.txt", false));
    }

    #[test]
    fn test_sum_spring_arrangements_test03() {
        assert_eq!(4, sum_spring_arrangements("input/2023/day12_test03.txt", false));
    }

    #[test]
    fn test_sum_spring_arrangements_test04() {
        assert_eq!(1, sum_spring_arrangements("input/2023/day12_test04.txt", false));
    }

    #[test]
    fn test_sum_spring_arrangements_test05() {
        assert_eq!(1, sum_spring_arrangements("input/2023/day12_test05.txt", false));
    }

    #[test]
    fn test_sum_spring_arrangements_test06() {
        assert_eq!(4, sum_spring_arrangements("input/2023/day12_test06.txt", false));
    }

    #[test]
    fn test_sum_spring_arrangements_test07() {
        assert_eq!(10, sum_spring_arrangements("input/2023/day12_test07.txt", false));
    }

    #[test]
    fn test_sum_spring_arrangements_part01() {
        assert_eq!(6958, sum_spring_arrangements("input/2023/day12.txt", false));
    }

    #[test]
    fn test_sum_spring_arrangements_part02() {
        assert_eq!(
            6555315065024,
            sum_spring_arrangements("input/2023/day12.txt", true)
        );
    }
}
