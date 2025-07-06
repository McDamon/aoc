// https://adventofcode.com/2019/day/2

#[cfg(test)]
mod tests {
    use crate::intcode::parse_intcode_input;
    use crate::intcode::run_intcode;

    #[test]
    fn test_run_intcode_test01() {
        let expected_intcode = vec![1, 0, 4, 0, 99];
        let mut input_intcode = parse_intcode_input("input/2019/day05_test01.txt");
        let mut outputs = vec![];
        assert_eq!(expected_intcode, run_intcode(&mut input_intcode, 0, Some(1), &mut outputs));
    }

    #[test]
    fn test_run_intcode_test02() {
        let expected_intcode = vec![1002, 4, 3, 4, 99];
        let mut input_intcode = parse_intcode_input("input/2019/day05_test02.txt");
        let mut outputs = vec![];
        assert_eq!(expected_intcode, run_intcode(&mut input_intcode, 0, None, &mut outputs));
    }
    
    #[test]
    fn test_run_intcode_test03() {
        let expected_intcode = vec![1101, 100, -1, 4, 99];
        let mut input_intcode = parse_intcode_input("input/2019/day05_test03.txt");
        let mut outputs = vec![];
        assert_eq!(expected_intcode, run_intcode(&mut input_intcode, 0, None, &mut outputs));
    }

    #[test]
    fn test_run_diagnostic_code() {
        let mut input_intcode = parse_intcode_input("input/2019/day05.txt");
        let mut outputs = vec![];
        run_intcode(&mut input_intcode, 0, Some(1), &mut outputs);
        assert_eq!(*outputs.last().unwrap(), 4601506);
    }
}