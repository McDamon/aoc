// https://adventofcode.com/2019/day/1

use crate::utils::get_lines;

struct Input {
    masses: Vec<u32>
}

fn parse_input(input_file: &str) -> Input {
    let lines = get_lines(input_file);

    let mut masses: Vec<u32> = Vec::new();

    for line in lines {
        masses.push(line.parse::<u32>().unwrap());
    }

    Input {
        masses
    }
}

pub fn get_sum_fuel_reqs(input_file: &str) -> u32 {
    let input = parse_input(input_file);

    let mut sum_fuel_reqs = 0;

    for mass in input.masses {
        sum_fuel_reqs += (mass / 3).saturating_sub(2);
    }

    sum_fuel_reqs
}

pub fn get_sum_fuel_reqs_with_fuel(input_file: &str) -> u32 {
    let input = parse_input(input_file);

    let mut sum_fuel_reqs = 0;

    for mass in input.masses {
        let mut fuel = (mass / 3).saturating_sub(2);
        while fuel > 0 {
            sum_fuel_reqs += fuel;
            fuel = (fuel / 3).saturating_sub(2);
        }
    }

    sum_fuel_reqs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_sum_fuel_reqs_test01() {
        assert_eq!(34241, get_sum_fuel_reqs("input/2019/day01_test01.txt"));
    }

    #[test]
    fn test_get_sum_fuel_reqs() {
        assert_eq!(3361976, get_sum_fuel_reqs("input/2019/day01.txt"));
    }

    #[test]
    fn test_get_sum_fuel_reqs_with_fuel_test02() {
        assert_eq!(51314, get_sum_fuel_reqs_with_fuel("input/2019/day01_test02.txt"));
    }

    #[test]
    fn test_get_sum_fuel_reqs_with_fuel() {
        assert_eq!(5040085, get_sum_fuel_reqs_with_fuel("input/2019/day01.txt"));
    }
}
