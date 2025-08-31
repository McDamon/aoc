// https://adventofcode.com/2019/day/8

use hashbrown::HashMap;
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

pub fn decode_image(input_file: &str, width: usize, height: usize) -> Vec<Vec<u32>> {
    let input = parse_input(input_file, width, height);

    let mut image_map: HashMap<(usize, usize), u32> = HashMap::new();
    for layer in input.layers.iter() {
        for (i, row) in layer.iter().enumerate() {
            for (j, pixel) in row.iter().enumerate() {
                if let Some(top_pixel) = image_map.get(&(i, j)) {
                    if *top_pixel == 2 {
                        image_map.insert((i, j), *pixel);
                    }
                } else {
                    image_map.insert((i, j), *pixel);
                }
            }
        }
    }

    let mut image = vec![];

    for i in 0..height {
        let mut row = vec![];
        for j in 0..width {
            if let Some(pixel) = image_map.get(&(i, j)) {
                if *pixel == 1 {
                    print!("{:?}", pixel);
                } else {
                    print!(" ");
                }
                row.push(*pixel);
            }
        }
        println!();
        image.push(row);
    }

    image
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

    #[test]
    fn test_decode_image_test02() {
        assert_eq!(
            vec![vec![0, 1], vec![1, 0]],
            decode_image("input/2019/day08_test02.txt", 2, 2)
        );
    }

    #[test]
    fn test_decode_image() {
        assert_eq!(
            vec![
                vec![
                    1, 1, 1, 1, 0, 1, 0, 0, 1, 0, 1, 0, 0, 1, 0, 0, 1, 1, 0, 0, 0, 0, 1, 1, 0
                ],
                vec![
                    0, 0, 0, 1, 0, 1, 0, 0, 1, 0, 1, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 1, 0
                ],
                vec![
                    0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 1, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0
                ],
                vec![
                    0, 1, 0, 0, 0, 1, 0, 0, 1, 0, 1, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0
                ],
                vec![
                    1, 0, 0, 0, 0, 1, 0, 0, 1, 0, 1, 0, 1, 0, 0, 1, 0, 0, 1, 0, 1, 0, 0, 1, 0
                ],
                vec![
                    1, 1, 1, 1, 0, 0, 1, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 1, 0, 0, 0, 1, 1, 0, 0
                ],
            ],
            decode_image("input/2019/day08.txt", 25, 6)
        );
    }
}
