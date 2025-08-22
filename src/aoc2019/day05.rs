// https://adventofcode.com/2019/day/5

#[cfg(test)]
mod tests {
    use crate::intcode::parse_intcode_input;
    use crate::intcode::run_intcode;

    #[test]
    fn test_run_intcode_test01() {
        let expected_intcode = vec![1, 0, 4, 0, 99];
        let mut input_intcode = parse_intcode_input("input/2019/day05_test01.txt");
        let mut outputs = vec![];
        assert_eq!(
            expected_intcode,
            run_intcode(&mut input_intcode, 0, Some(1), &mut outputs)
        );
    }

    #[test]
    fn test_run_intcode_test02() {
        let expected_intcode = vec![1002, 4, 3, 4, 99];
        let mut input_intcode = parse_intcode_input("input/2019/day05_test02.txt");
        let mut outputs = vec![];
        assert_eq!(
            expected_intcode,
            run_intcode(&mut input_intcode, 0, None, &mut outputs)
        );
    }

    #[test]
    fn test_run_intcode_test03() {
        let expected_intcode = vec![1101, 100, -1, 4, 99];
        let mut input_intcode = parse_intcode_input("input/2019/day05_test03.txt");
        let mut outputs = vec![];
        assert_eq!(
            expected_intcode,
            run_intcode(&mut input_intcode, 0, None, &mut outputs)
        );
    }

    #[test]
    fn test_run_diagnostic_code_part_one() {
        let mut input_intcode = parse_intcode_input("input/2019/day05.txt");
        let mut outputs = vec![];
        run_intcode(&mut input_intcode, 0, Some(1), &mut outputs);
        assert_eq!(*outputs.last().unwrap(), 4601506);
    }

    #[test]
    fn test_run_intcode_test04() {
        let expected_intcode_eq = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, 1, 8];
        let expected_intcode_neq = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, 0, 8];
        let mut input_intcode = parse_intcode_input("input/2019/day05_test04.txt");
        let mut outputs = vec![];
        assert_eq!(
            expected_intcode_eq,
            run_intcode(&mut input_intcode, 0, Some(8), &mut outputs)
        );
        assert_eq!(
            expected_intcode_neq,
            run_intcode(&mut input_intcode, 0, Some(7), &mut outputs)
        );
    }

    #[test]
    fn test_run_intcode_test05() {
        let expected_intcode_lt = vec![3,9,7,9,10,9,4,9,99,1,8];
        let expected_intcode_gt = vec![3,9,7,9,10,9,4,9,99,0,8];
        let mut input_intcode = parse_intcode_input("input/2019/day05_test05.txt");
        let mut outputs = vec![];
        assert_eq!(
            expected_intcode_lt,
            run_intcode(&mut input_intcode, 0, Some(7), &mut outputs)
        );
        assert_eq!(
            expected_intcode_gt,
            run_intcode(&mut input_intcode, 0, Some(9), &mut outputs)
        );
    }

    #[test]
    fn test_run_intcode_test06() {
        let expected_intcode_eq = vec![3,3,1108,1,8,3,4,3,99];
        let expected_intcode_neq = vec![3,3,1108,0,8,3,4,3,99];
        let mut input_intcode = parse_intcode_input("input/2019/day05_test06.txt");
        let mut outputs = vec![];
        assert_eq!(
            expected_intcode_eq,
            run_intcode(&mut input_intcode, 0, Some(8), &mut outputs)
        );
        assert_eq!(
            expected_intcode_neq,
            run_intcode(&mut input_intcode, 0, Some(7), &mut outputs)
        );
    }

    #[test]
    fn test_run_intcode_test07() {
        let expected_intcode_lt = vec![3,3,1107,1,8,3,4,3,99];
        let expected_intcode_gt = vec![3,3,1107,0,8,3,4,3,99];
        let mut input_intcode = parse_intcode_input("input/2019/day05_test07.txt");
        let mut outputs = vec![];
        assert_eq!(
            expected_intcode_lt,
            run_intcode(&mut input_intcode, 0, Some(7), &mut outputs)
        );
        assert_eq!(
            expected_intcode_gt,
            run_intcode(&mut input_intcode, 0, Some(9), &mut outputs)
        );
    }

    #[test]
    fn test_run_intcode_test08() {
        let expected_intcode_zero = vec![3,12,6,12,15,1,13,14,13,4,13,99,0,0,1,9];
        let expected_intcode_nonzero = vec![3,12,6,12,15,1,13,14,13,4,13,99,1,0,1,9];
        let mut input_intcode = parse_intcode_input("input/2019/day05_test08.txt");
        let mut outputs = vec![];
        assert_eq!(
            expected_intcode_zero,
            run_intcode(&mut input_intcode, 0, Some(0), &mut outputs)
        );
        assert_eq!(
            expected_intcode_nonzero,
            run_intcode(&mut input_intcode, 0, Some(1), &mut outputs)
        );
    }

    #[test]
    fn test_run_intcode_test09() {
        let expected_intcode_zero = vec![3,3,1105,1,9,1101,0,0,12,4,12,99,1];
        let expected_intcode_nonzero = vec![3,3,1105,0,9,1101,0,0,12,4,12,99,1];
        let mut input_intcode = parse_intcode_input("input/2019/day05_test09.txt");
        let mut outputs = vec![];
        assert_eq!(
            expected_intcode_zero,
            run_intcode(&mut input_intcode, 0, Some(0), &mut outputs)
        );
        assert_eq!(
            expected_intcode_nonzero,
            run_intcode(&mut input_intcode, 0, Some(1), &mut outputs)
        );
    }

    #[test]
    fn test_run_intcode_test10() {
        let expected_intcode_lt = vec![3,3,1105,1,9,1101,0,0,12,4,12,99,1];
        let expected_intcode_eq = vec![3,3,1105,0,9,1101,0,0,12,4,12,99,1];
        let expected_intcode_gt = vec![3,3,1105,0,9,1101,0,0,12,4,12,99,1];
        let mut input_intcode = parse_intcode_input("input/2019/day05_test10.txt");
        let mut outputs = vec![];
        assert_eq!(
            expected_intcode_lt,
            run_intcode(&mut input_intcode, 0, Some(7), &mut outputs)
        );
        assert_eq!(
            expected_intcode_eq,
            run_intcode(&mut input_intcode, 0, Some(8), &mut outputs)
        );
        assert_eq!(
            expected_intcode_gt,
            run_intcode(&mut input_intcode, 0, Some(9), &mut outputs)
        );
    }

    #[test]
    fn test_run_diagnostic_code_part_two() {
        let mut input_intcode = parse_intcode_input("input/2019/day05.txt");
        let mut outputs = vec![];
        run_intcode(&mut input_intcode, 0, Some(5), &mut outputs);
        assert_eq!(*outputs.last().unwrap(), 0);
    }
}
