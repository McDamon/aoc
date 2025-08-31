// https://adventofcode.com/2019/day/9

#[cfg(test)]
mod tests {
    use crate::intcode::parse_intcode_input;
    use crate::intcode::run_intcode;

    #[test]
    fn test_run_intcode_test01() {
        let expected_intcode = vec![
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ];
        let mut input_intcode = parse_intcode_input("input/2019/day09_test01.txt");
        input_intcode.extend(vec![0; 1000]);
        let mut outputs = vec![];
        run_intcode(
            &mut input_intcode,
            &mut 0,
            &mut 0,
            &mut vec![],
            &mut outputs,
        );
        assert_eq!(expected_intcode, outputs)
    }

    #[test]
    fn test_run_intcode_test02() {
        let expected_intcode = vec![1102, 34915192, 34915192, 7, 4, 7, 99, 1219070632396864];
        let mut input_intcode = parse_intcode_input("input/2019/day09_test02.txt");
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
        assert_eq!(vec![1219070632396864], outputs)
    }

    #[test]
    fn test_run_intcode_test03() {
        let expected_intcode = vec![104, 1125899906842624, 99];
        let mut input_intcode = parse_intcode_input("input/2019/day09_test03.txt");
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
        assert_eq!(vec![1125899906842624], outputs)
    }

    #[test]
    fn test_run_intcode_part01() {
        let mut input_intcode = parse_intcode_input("input/2019/day09.txt");
        input_intcode.extend(vec![0; 1000]);
        let mut outputs = vec![];
        run_intcode(
            &mut input_intcode,
            &mut 0,
            &mut 0,
            &mut vec![1],
            &mut outputs,
        );
        println!("Outputs: {:?}", outputs);
    }
}
