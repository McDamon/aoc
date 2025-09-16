// https://adventofcode.com/2019/day/11

use std::collections::HashSet;

use crate::{intcode::run_intcode, utils::Direction};

struct RobotPose {
    pos: (usize, usize),
    dir: Direction,
}

pub fn get_painted_panels(intcode: &mut Vec<isize>) -> usize {
    let mut outputs = vec![];
    let mut grid: HashSet<(usize, usize)> = HashSet::new();

    let mut prog_counter: usize = 0usize;

    run_intcode(
        intcode,
        &mut prog_counter,
        &mut 0,
        &mut vec![],
        &mut outputs,
    );

    0
}

#[cfg(test)]
mod tests {
    use crate::aoc2019::day11::get_painted_panels;
    use crate::intcode::parse_intcode_input;

    #[test]
    fn test_run_intcode() {
        let mut input_intcode = parse_intcode_input("input/2019/day11.txt");
        let painted_panels = get_painted_panels(&mut input_intcode);
        assert_eq!(0, painted_panels);
    }
}
