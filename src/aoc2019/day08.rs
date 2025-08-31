// https://adventofcode.com/2019/day/8

use itertools::Itertools;

use crate::utils::get_lines;

struct Input {
    layers: Vec<Vec<Vec<u32>>>,
}

fn parse_input(input_file: &str, width: usize, height: usize) -> Input {
    let lines = get_lines(input_file);

    let mut layers: Vec<Vec<Vec<u32>>> = Vec::new();

    if let Some(image) = lines.first() {
        for layer in &image.chars().chunks(width * height) {
            let layer_str = layer.collect::<String>();
            let mut layer_vec = vec![];
            for row in &layer_str.chars().chunks(width) {
                let row_str = row.collect::<String>();
                let row_vec = row_str
                    .chars()
                    .map(|c| c.to_digit(10).unwrap())
                    .collect_vec();
                layer_vec.push(row_vec);
            }
            layers.push(layer_vec);
        }
    }

    Input { layers }
}

pub fn get_layer_with_fewest_zeros(layers: &[Vec<Vec<u32>>]) -> Option<Vec<Vec<u32>>> {
    let mut zero_digits_count_by_layer: Vec<(u32, usize)> = vec![];
    for (i, layer) in layers.iter().enumerate() {
        let flat_layer: Vec<u32> = layer.iter().flatten().copied().collect();
        let num_zero_digits = flat_layer.into_iter().filter(|&c| c == 0).count() as u32;
        zero_digits_count_by_layer.push((num_zero_digits, i));
    }
    zero_digits_count_by_layer.sort();
    if let Some((_, i)) = zero_digits_count_by_layer.first() {
        Some(layers[*i].clone())
    } else {
        None
    }
}

pub fn get_layer_num_of_one_digits_mult_num_two_digits(layer: &[Vec<u32>]) -> u32 {
    let flat_layer: Vec<u32> = layer.iter().flatten().copied().collect();
    let num_one_digits = flat_layer.iter().filter(|&&c| c == 1).count() as u32;
    let num_two_digits = flat_layer.iter().filter(|&&c| c == 2).count() as u32;
    num_one_digits * num_two_digits
}

pub fn get_num_of_one_digits_mult_num_two_digits(
    input_file: &str,
    width: usize,
    height: usize,
) -> u32 {
    let input = parse_input(input_file, width, height);

    let layer_with_fewest_zeros = get_layer_with_fewest_zeros(&input.layers);

    match layer_with_fewest_zeros {
        Some(layer) => get_layer_num_of_one_digits_mult_num_two_digits(&layer),
        None => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_num_of_one_digits_mult_num_two_digits_test01() {
        assert_eq!(
            1,
            get_num_of_one_digits_mult_num_two_digits("input/2019/day08_test01.txt", 3, 2)
        );
    }

    #[test]
    fn test_get_num_of_one_digits_mult_num_two_digits() {
        assert_eq!(
            1820,
            get_num_of_one_digits_mult_num_two_digits("input/2019/day08.txt", 25, 6)
        );
    }
}
