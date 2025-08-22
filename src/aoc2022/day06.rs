// https://adventofcode.com/2022/day/6

use crate::utils::get_lines;

pub fn get_num_chars_before_sop_marker<const N: usize>(input: &str) -> i32 {
    let mut array = Vec::<char>::with_capacity(N);

    let mut count = N;

    for c in input.chars() {
        if array.len() < N {
            array.push(c);
        } else if array.len() == N {
            if is_unique(array.as_slice()) {
                break;
            }
            array.remove(0);
            array.push(c);
            count += 1;
        }
    }

    count as i32
}

pub fn is_unique<T: PartialEq>(slice: &[T]) -> bool {
    for i in 1..slice.len() {
        if slice[i..].contains(&slice[i - 1]) {
            return false;
        }
    }
    true
}

pub fn get_num_chars_before_sop_marker_file<const N: usize>(input_file: &str) -> i32 {
    let lines = get_lines(input_file);

    let num_chars = get_num_chars_before_sop_marker::<N>(&lines[0]);
    num_chars
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_num_chars_before_sop_marker_1() {
        assert_eq!(
            7,
            get_num_chars_before_sop_marker::<4>("mjqjpqmgbljsphdztnvjfqwrcgsmlb")
        );
    }

    #[test]
    fn test_get_num_chars_before_sop_marker_2() {
        assert_eq!(
            5,
            get_num_chars_before_sop_marker::<4>("bvwbjplbgvbhsrlpgdmjqwftvncz")
        );
    }

    #[test]
    fn test_get_num_chars_before_sop_marker_3() {
        assert_eq!(
            6,
            get_num_chars_before_sop_marker::<4>("nppdvjthqldpwncqszvftbrmjlhg")
        );
    }

    #[test]
    fn test_get_num_chars_before_sop_marker_4() {
        assert_eq!(
            10,
            get_num_chars_before_sop_marker::<4>("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")
        );
    }

    #[test]
    fn test_get_num_chars_before_sop_marker_5() {
        assert_eq!(
            11,
            get_num_chars_before_sop_marker::<4>("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")
        );
    }

    #[test]
    fn test_get_num_chars_before_sop_marker_alt_1() {
        assert_eq!(
            19,
            get_num_chars_before_sop_marker::<14>("mjqjpqmgbljsphdztnvjfqwrcgsmlb")
        );
    }

    #[test]
    fn test_get_num_chars_before_sop_marker_alt_2() {
        assert_eq!(
            23,
            get_num_chars_before_sop_marker::<14>("bvwbjplbgvbhsrlpgdmjqwftvncz")
        );
    }

    #[test]
    fn test_get_num_chars_before_sop_marker_alt_3() {
        assert_eq!(
            23,
            get_num_chars_before_sop_marker::<14>("nppdvjthqldpwncqszvftbrmjlhg")
        );
    }

    #[test]
    fn test_get_num_chars_before_sop_marker_alt_4() {
        assert_eq!(
            29,
            get_num_chars_before_sop_marker::<14>("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")
        );
    }

    #[test]
    fn test_get_num_chars_before_sop_marker_alt_5() {
        assert_eq!(
            26,
            get_num_chars_before_sop_marker::<14>("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")
        );
    }

    #[test]
    fn test_get_num_chars_before_sop_marker() {
        assert_eq!(
            1833,
            get_num_chars_before_sop_marker_file::<4>("input/2022/day06.txt")
        );
    }

    #[test]
    fn test_get_num_chars_before_sop_marker_alt() {
        assert_eq!(
            3425,
            get_num_chars_before_sop_marker_file::<14>("input/2022/day06.txt")
        );
    }
}
