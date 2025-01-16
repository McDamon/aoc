// https://adventofcode.com/2024/day/19

use std::collections::HashMap;

use crate::utils::get_lines;

#[derive(Debug)]
struct Input {
    towels: Vec<String>,
    designs: Vec<String>,
}

fn parse_input(input_file: &str) -> Input {
    let lines = get_lines(input_file);

    let mut towels: Vec<String> = if let Some(first_line) = lines.first() {
        first_line
            .split(',')
            .map(|s| s.trim().to_string())
            .collect()
    } else {
        panic!("No towels found");
    };

    towels.sort_by(|a, b| b.cmp(a));

    let designs = lines.iter().skip(2).map(|line| line.to_string()).collect();

    Input { towels, designs }
}

fn poss_design_combos(
    towels: &Vec<String>,
    design: &str,
    memo: &mut HashMap<String, usize>,
) -> usize {
    let mut num_combos = 0;
    if design.is_empty() {
        num_combos += 1;
    } else if memo.contains_key(design) {
        num_combos += memo[design];
    } else {
        for towel in towels {
            if design.starts_with(towel) {
                num_combos += poss_design_combos(towels, &design[towel.len()..], memo);
            }
        }
        memo.insert(design.to_string(), num_combos);
    }
    num_combos
}

fn get_poss_designs(input_file: &str) -> (usize, usize) {
    let input = parse_input(input_file);

    //println!("towels: {:?}", input.towels);
    //println!("designs: {:?}", input.designs);

    let mut poss_designs = 0;
    let mut memo = HashMap::new();
    let mut tot_num_combos = 0;
    for design in &input.designs {
        let num_combos = poss_design_combos(&input.towels, design, &mut memo);
        if num_combos > 0 {
            poss_designs += 1;
        }
        tot_num_combos += num_combos;
    }

    (poss_designs, tot_num_combos)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_poss_designs_test01() {
        assert_eq!(6, get_poss_designs("input/2024/day19_test01.txt").0);
    }

    #[test]
    fn test_get_poss_designs_test02() {
        assert_eq!(1, get_poss_designs("input/2024/day19_test02.txt").0);
    }

    #[test]
    fn test_get_poss_designs_test03() {
        assert_eq!(0, get_poss_designs("input/2024/day19_test03.txt").0);
    }

    #[test]
    fn test_get_poss_designs_test04() {
        assert_eq!(0, get_poss_designs("input/2024/day19_test04.txt").0);
    }

    #[test]
    fn test_get_poss_designs() {
        assert_eq!(317, get_poss_designs("input/2024/day19.txt").0);
    }

    #[test]
    fn test_get_num_combos_test01() {
        assert_eq!(16, get_poss_designs("input/2024/day19_test01.txt").1);
    }

    #[test]
    fn test_get_num_combos() {
        assert_eq!(883443544805484, get_poss_designs("input/2024/day19.txt").1);
    }
}
