// https://adventofcode.com/2019/day/2

#[cfg(test)]
mod tests {
    use crate::intcode::parse_intcode_input;
    use crate::intcode::run_intcode;

    #[test]
    fn test_run_intcode_test01() {
        let expected_intcode = vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50];
        let mut input_intcode = parse_intcode_input("input/2019/day02_test01.txt");
        assert_eq!(expected_intcode, run_intcode(&mut input_intcode, 0));
    }

    #[test]
    fn test_run_intcode_test02() {
        let expected_intcode = vec![2, 0, 0, 0, 99];
        let mut input_intcode = parse_intcode_input("input/2019/day02_test02.txt");
        assert_eq!(expected_intcode, run_intcode(&mut input_intcode, 0));
    }

    #[test]
    fn test_run_intcode_test03() {
        let expected_intcode = vec![2, 3, 0, 6, 99];
        let mut input_intcode = parse_intcode_input("input/2019/day02_test03.txt");
        assert_eq!(expected_intcode, run_intcode(&mut input_intcode, 0));
    }

    #[test]
    fn test_run_intcode_test04() {
        let expected_intcode = vec![2, 4, 4, 5, 99, 9801];
        let mut input_intcode = parse_intcode_input("input/2019/day02_test04.txt");
        assert_eq!(expected_intcode, run_intcode(&mut input_intcode, 0));
    }

    #[test]
    fn test_run_intcode_test05() {
        let expected_intcode = vec![30, 1, 1, 4, 2, 5, 6, 0, 99];
        let mut input_intcode = parse_intcode_input("input/2019/day02_test05.txt");
        assert_eq!(expected_intcode, run_intcode(&mut input_intcode, 0));
    }

    #[test]
    fn test_run_intcode_part01() {
        let mut input_intcode = parse_intcode_input("input/2019/day02.txt");
        input_intcode[1] = 12;
        input_intcode[2] = 2;
        let result_intcode = run_intcode(&mut input_intcode, 0);
        assert_eq!(10566835, result_intcode[0]);
    }

    #[test]
    fn test_run_intcode_part02() {
        let input_intcode = parse_intcode_input("input/2019/day02.txt");
        for noun in 0..99 {
            for verb in 0..99 {
                let mut intcode = input_intcode.clone();
                intcode[1] = noun;
                intcode[2] = verb;
                let result_intcode = run_intcode(&mut intcode, 0);
                if result_intcode[0] == 19690720 {
                    assert_eq!(2347, 100 * noun + verb)
                }
            }
        }
    }
}
