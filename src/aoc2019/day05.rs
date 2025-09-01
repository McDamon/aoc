// https://adventofcode.com/2019/day/5

#[cfg(test)]
mod tests {
    use crate::intcode::parse_intcode_input;
    use crate::intcode::run_intcode;

    #[test]
    fn test_run_intcode_test01() {
        let mut input_intcode = parse_intcode_input("input/2019/day05_test01.txt");
        let mut outputs = vec![];
        run_intcode(
            &mut input_intcode,
            &mut 0,
            &mut 0,
            &mut vec![1],
            &mut outputs,
        );
        assert_eq!(vec![1], outputs)
    }

    #[test]
    fn test_run_intcode_test02() {
        let expected_intcode = vec![1002, 4, 3, 4, 99];
        let mut input_intcode = parse_intcode_input("input/2019/day05_test02.txt");
        let mut outputs = vec![];
        assert_eq!(
            expected_intcode,
            run_intcode(
                &mut input_intcode,
                &mut 0,
                &mut 0,
                &mut vec![],
                &mut outputs
            )
        );
    }

    #[test]
    fn test_run_intcode_test03() {
        let expected_intcode = vec![1101, 100, -1, 4, 99];
        let mut input_intcode = parse_intcode_input("input/2019/day05_test03.txt");
        let mut outputs = vec![];
        assert_eq!(
            expected_intcode,
            run_intcode(
                &mut input_intcode,
                &mut 0,
                &mut 0,
                &mut vec![],
                &mut outputs
            )
        );
    }

    #[test]
    fn test_run_diagnostic_code_part_one() {
        let mut input_intcode = parse_intcode_input("input/2019/day05.txt");
        let mut outputs = vec![];
        run_intcode(
            &mut input_intcode,
            &mut 0,
            &mut 0,
            &mut vec![1],
            &mut outputs,
        );
        assert_eq!(*outputs.last().unwrap(), 4601506);
    }

    #[test]
    fn test_run_intcode_test04_eq() {
        let expected_intcode_eq = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, 1, 8];
        let mut input_intcode = parse_intcode_input("input/2019/day05_test04.txt");
        let mut outputs = vec![];
        assert_eq!(
            expected_intcode_eq,
            run_intcode(
                &mut input_intcode,
                &mut 0,
                &mut 0,
                &mut vec![8],
                &mut outputs
            )
        );
        assert_eq!(*outputs.last().unwrap(), 1);
    }

    #[test]
    fn test_run_intcode_test04_neq() {
        let expected_intcode_neq = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, 0, 8];
        let mut input_intcode = parse_intcode_input("input/2019/day05_test04.txt");
        let mut outputs = vec![];
        assert_eq!(
            expected_intcode_neq,
            run_intcode(
                &mut input_intcode,
                &mut 0,
                &mut 0,
                &mut vec![7],
                &mut outputs
            )
        );
        assert_eq!(*outputs.last().unwrap(), 0);
    }

    #[test]
    fn test_run_intcode_test05_lt() {
        let expected_intcode_lt = vec![3, 9, 7, 9, 10, 9, 4, 9, 99, 1, 8];
        let mut input_intcode = parse_intcode_input("input/2019/day05_test05.txt");
        let mut outputs = vec![];
        assert_eq!(
            expected_intcode_lt,
            run_intcode(
                &mut input_intcode,
                &mut 0,
                &mut 0,
                &mut vec![7],
                &mut outputs
            )
        );
        assert_eq!(*outputs.last().unwrap(), 1);
    }

    #[test]
    fn test_run_intcode_test05_gt() {
        let expected_intcode_gt = vec![3, 9, 7, 9, 10, 9, 4, 9, 99, 0, 8];
        let mut input_intcode = parse_intcode_input("input/2019/day05_test05.txt");
        let mut outputs = vec![];
        assert_eq!(
            expected_intcode_gt,
            run_intcode(
                &mut input_intcode,
                &mut 0,
                &mut 0,
                &mut vec![9],
                &mut outputs
            )
        );
        assert_eq!(*outputs.last().unwrap(), 0);
    }

    #[test]
    fn test_run_intcode_test06_eq() {
        let expected_intcode_eq = vec![3, 3, 1108, 1, 8, 3, 4, 3, 99];
        let mut input_intcode = parse_intcode_input("input/2019/day05_test06.txt");
        let mut outputs = vec![];
        assert_eq!(
            expected_intcode_eq,
            run_intcode(
                &mut input_intcode,
                &mut 0,
                &mut 0,
                &mut vec![8],
                &mut outputs
            )
        );
        assert_eq!(*outputs.last().unwrap(), 1);
    }

    #[test]
    fn test_run_intcode_test06_neq() {
        let expected_intcode_neq = vec![3, 3, 1108, 0, 8, 3, 4, 3, 99];
        let mut input_intcode = parse_intcode_input("input/2019/day05_test06.txt");
        let mut outputs = vec![];
        assert_eq!(
            expected_intcode_neq,
            run_intcode(
                &mut input_intcode,
                &mut 0,
                &mut 0,
                &mut vec![7],
                &mut outputs
            )
        );
        assert_eq!(*outputs.last().unwrap(), 0);
    }

    #[test]
    fn test_run_intcode_test07_lt() {
        let expected_intcode_lt = vec![3, 3, 1107, 1, 8, 3, 4, 3, 99];
        let mut input_intcode = parse_intcode_input("input/2019/day05_test07.txt");
        let mut outputs = vec![];
        assert_eq!(
            expected_intcode_lt,
            run_intcode(
                &mut input_intcode,
                &mut 0,
                &mut 0,
                &mut vec![7],
                &mut outputs
            )
        );
        assert_eq!(*outputs.last().unwrap(), 1);
    }

    #[test]
    fn test_run_intcode_test07_gt() {
        let expected_intcode_gt = vec![3, 3, 1107, 0, 8, 3, 4, 3, 99];
        let mut input_intcode = parse_intcode_input("input/2019/day05_test07.txt");
        let mut outputs = vec![];
        assert_eq!(
            expected_intcode_gt,
            run_intcode(
                &mut input_intcode,
                &mut 0,
                &mut 0,
                &mut vec![9],
                &mut outputs
            )
        );
        assert_eq!(*outputs.last().unwrap(), 0);
    }

    #[test]
    fn test_run_intcode_test08_zero() {
        let expected_intcode_zero = vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, 0, 0, 1, 9];
        let mut input_intcode = parse_intcode_input("input/2019/day05_test08.txt");
        let mut outputs = vec![];
        assert_eq!(
            expected_intcode_zero,
            run_intcode(
                &mut input_intcode,
                &mut 0,
                &mut 0,
                &mut vec![0],
                &mut outputs
            )
        );
        assert_eq!(*outputs.last().unwrap(), 0);
    }

    #[test]
    fn test_run_intcode_test08_nonzero() {
        let expected_intcode_nonzero = vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, 1, 1, 1, 9];
        let mut input_intcode = parse_intcode_input("input/2019/day05_test08.txt");
        let mut outputs = vec![];
        assert_eq!(
            expected_intcode_nonzero,
            run_intcode(
                &mut input_intcode,
                &mut 0,
                &mut 0,
                &mut vec![1],
                &mut outputs
            )
        );
        assert_eq!(*outputs.last().unwrap(), 1);
    }

    #[test]
    fn test_run_intcode_test09_zero() {
        let expected_intcode_zero = vec![3, 3, 1105, 0, 9, 1101, 0, 0, 12, 4, 12, 99, 0];
        let mut input_intcode = parse_intcode_input("input/2019/day05_test09.txt");
        let mut outputs = vec![];
        assert_eq!(
            expected_intcode_zero,
            run_intcode(
                &mut input_intcode,
                &mut 0,
                &mut 0,
                &mut vec![0],
                &mut outputs
            )
        );
        assert_eq!(*outputs.last().unwrap(), 0);
    }

    #[test]
    fn test_run_intcode_test09_nonzero() {
        let expected_intcode_nonzero = vec![3, 3, 1105, 1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];
        let mut input_intcode = parse_intcode_input("input/2019/day05_test09.txt");
        let mut outputs = vec![];
        assert_eq!(
            expected_intcode_nonzero,
            run_intcode(
                &mut input_intcode,
                &mut 0,
                &mut 0,
                &mut vec![1],
                &mut outputs
            )
        );
        assert_eq!(*outputs.last().unwrap(), 1);
    }

    #[test]
    fn test_run_intcode_test10_lt() {
        let expected_intcode_lt = vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            7, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ];
        let mut input_intcode = parse_intcode_input("input/2019/day05_test10.txt");
        let mut outputs = vec![];
        assert_eq!(
            expected_intcode_lt,
            run_intcode(
                &mut input_intcode,
                &mut 0,
                &mut 0,
                &mut vec![7],
                &mut outputs
            )
        );
        assert_eq!(*outputs.last().unwrap(), 999);
    }

    #[test]
    fn test_run_intcode_test10_eq() {
        let expected_intcode_eq = vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98,
            1000, 8, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1,
            20, 4, 20, 1105, 1, 46, 98, 99,
        ];
        let mut input_intcode = parse_intcode_input("input/2019/day05_test10.txt");
        let mut outputs = vec![];
        assert_eq!(
            expected_intcode_eq,
            run_intcode(
                &mut input_intcode,
                &mut 0,
                &mut 0,
                &mut vec![8],
                &mut outputs
            )
        );
        assert_eq!(*outputs.last().unwrap(), 1000);
    }

    #[test]
    fn test_run_intcode_test10_gt() {
        let expected_intcode_gt = vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98,
            1001, 9, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1,
            20, 4, 20, 1105, 1, 46, 98, 99,
        ];
        let mut input_intcode = parse_intcode_input("input/2019/day05_test10.txt");
        let mut outputs = vec![];
        assert_eq!(
            expected_intcode_gt,
            run_intcode(
                &mut input_intcode,
                &mut 0,
                &mut 0,
                &mut vec![9],
                &mut outputs
            )
        );
        assert_eq!(*outputs.last().unwrap(), 1001);
    }

    #[test]
    fn test_run_diagnostic_code_part_two() {
        let mut input_intcode = parse_intcode_input("input/2019/day05.txt");
        let mut outputs = vec![];
        run_intcode(
            &mut input_intcode,
            &mut 0,
            &mut 0,
            &mut vec![5],
            &mut outputs,
        );
        assert_eq!(*outputs.last().unwrap(), 5525561);
    }
}
