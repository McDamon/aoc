// https://adventofcode.com/2024/day/9

use std::collections::HashSet;

use super::utils::get_lines;

struct InputPartOne {
    disk: Vec<DiskEntry>,
}

struct InputPartTwo {
    disk: Vec<DiskEntryWithLen>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum DiskEntryType {
    File,
    FreeSpace,
}

#[derive(Debug, Clone)]
struct DiskEntry {
    id: Option<usize>,
    entry: DiskEntryType,
}

#[derive(Debug, Clone, Copy)]
struct DiskEntryWithLen {
    id: Option<usize>,
    entry: DiskEntryType,
    len: usize,
}

fn parse_input_part_one(input_file: &str) -> InputPartOne {
    let lines = get_lines(input_file);

    if let Some(line) = lines.first() {
        let chars: Vec<char> = line.chars().collect();

        let mut id = 0usize;
        let disk: Vec<DiskEntry> =
            chars
                .into_iter()
                .enumerate()
                .fold(vec![], |mut acc: Vec<DiskEntry>, (i, c)| {
                    if i % 2 != 0 {
                        id += 1;
                    }
                    for _ in 0..c.to_digit(10).unwrap_or(0) {
                        acc.push(DiskEntry {
                            id: if i % 2 == 0 { Some(id) } else { None },
                            entry: if i % 2 == 0 {
                                DiskEntryType::File
                            } else {
                                DiskEntryType::FreeSpace
                            },
                        });
                    }
                    acc
                });
        InputPartOne { disk }
    } else {
        panic!("Invalid input file: {}", input_file);
    }
}

fn parse_input_part_two(input_file: &str) -> InputPartTwo {
    let lines = get_lines(input_file);

    if let Some(line) = lines.first() {
        let chars: Vec<char> = line.chars().collect();

        let mut id = 0usize;
        let disk: Vec<DiskEntryWithLen> =
            chars
                .into_iter()
                .enumerate()
                .fold(vec![], |mut acc: Vec<DiskEntryWithLen>, (i, c)| {
                    if i % 2 != 0 {
                        id += 1;
                    }
                    acc.push(DiskEntryWithLen {
                        id: if i % 2 == 0 { Some(id) } else { None },
                        entry: if i % 2 == 0 {
                            DiskEntryType::File
                        } else {
                            DiskEntryType::FreeSpace
                        },
                        len: c.to_digit(10).unwrap() as usize,
                    });
                    acc
                });
        InputPartTwo { disk }
    } else {
        panic!("Invalid input file: {}", input_file);
    }
}

fn find_first_free_space_block(blocks: &[DiskEntry]) -> Option<usize> {
    blocks
        .iter()
        .position(|block| matches!(block.entry, DiskEntryType::FreeSpace))
}

fn find_last_file_block(blocks: &[DiskEntry]) -> Option<usize> {
    blocks
        .iter()
        .rposition(|block| matches!(block.entry, DiskEntryType::File))
}

fn has_file_block_gaps(blocks: &[DiskEntry]) -> bool {
    let free_space_count_at_end = blocks
        .iter()
        .rev()
        .take_while(|block| block.entry == DiskEntryType::FreeSpace)
        .count();
    let free_space_count = blocks
        .iter()
        .filter(|block| block.entry == DiskEntryType::FreeSpace)
        .count();
    free_space_count_at_end != free_space_count
}

fn calc_checksum(blocks: &[DiskEntry]) -> usize {
    blocks
        .iter()
        .enumerate()
        .filter(|(_, block)| block.entry != DiskEntryType::FreeSpace)
        .map(|(i, block)| i * block.id.unwrap_or(0))
        .sum()
}

fn get_checksum(input_file: &str) -> usize {
    let input = parse_input_part_one(input_file);

    let mut blocks = input.disk.clone();

    loop {
        if has_file_block_gaps(&blocks) {
            if let Some(first_free_space_block_pos) = find_first_free_space_block(&blocks)
                && let Some(last_file_block_pos) = find_last_file_block(&blocks)
            {
                let first_free_space_block = blocks[first_free_space_block_pos].clone();
                let last_file_block = blocks[last_file_block_pos].clone();

                blocks[first_free_space_block_pos] = last_file_block.clone();
                blocks[last_file_block_pos] = first_free_space_block.clone();
            }
        } else {
            break;
        }
    }

    //println!("{:?}", blocks);

    calc_checksum(&blocks)
}

fn print_blocks(blocks: &Vec<DiskEntryWithLen>) {
    for block in blocks {
        match block {
            DiskEntryWithLen {
                id: Some(id),
                entry: DiskEntryType::File,
                len,
            } => print!("{}", id.to_string().repeat(*len)),
            DiskEntryWithLen {
                id: None,
                entry: DiskEntryType::FreeSpace,
                len,
            } => print!("{}", ".".repeat(*len)),
            _ => (),
        }
    }
    println!()
}

fn find_first_whole_free_space(
    blocks: &[DiskEntryWithLen],
    required_len: usize,
    last_whole_file_pos: usize,
) -> Option<(usize, &DiskEntryWithLen)> {
    blocks.iter().enumerate().find(|(i, block)| {
        matches!(block.entry, DiskEntryType::FreeSpace)
            && block.len >= required_len
            && *i < last_whole_file_pos
    })
}

fn find_last_whole_file<'a>(
    blocks: &'a [DiskEntryWithLen],
    ignored_file_ids: &HashSet<usize>,
) -> Option<(usize, &'a DiskEntryWithLen)> {
    blocks.iter().enumerate().rev().find(|(_, block)| {
        matches!(block.entry, DiskEntryType::File) && !ignored_file_ids.contains(&block.id.unwrap())
    })
}

fn calc_checksum_whole_files(blocks: &[DiskEntryWithLen]) -> usize {
    let mut block_index = 0usize;
    let mut checksum = 0usize;
    for block in blocks.iter() {
        for _ in 0..block.len {
            if block.entry == DiskEntryType::File {
                checksum += block_index * block.id.unwrap();
            }
            block_index += 1;
        }
    }
    checksum
}

fn get_checksum_whole_files(input_file: &str) -> usize {
    let input = parse_input_part_two(input_file);

    let mut blocks = input.disk.clone();

    let mut ignored_file_ids: HashSet<usize> = HashSet::new();

    while let Some((last_whole_file_pos, &last_whole_file)) =
        find_last_whole_file(&blocks, &ignored_file_ids)
    {
        if let Some((first_free_space_pos, &first_free_space)) =
            find_first_whole_free_space(&blocks, last_whole_file.len, last_whole_file_pos)
        {
            if first_free_space.len >= last_whole_file.len {
                /*println!(
                    "Found free space block pos at {} with len {}, moving file block id {} at {} with len {}",
                    first_free_space_pos,
                    first_free_space.len,
                    last_whole_file.id.unwrap(),
                    last_whole_file_pos,
                    last_whole_file.len
                );
                println!("BEFORE");
                print_blocks(&blocks);

                println!("DURING");
                */

                blocks.remove(last_whole_file_pos);
                //print_blocks(&blocks);
                blocks.remove(first_free_space_pos);
                //print_blocks(&blocks);
                blocks.insert(
                    last_whole_file_pos - 1,
                    DiskEntryWithLen {
                        id: None,
                        entry: DiskEntryType::FreeSpace,
                        len: last_whole_file.len,
                    },
                );
                //print_blocks(&blocks);
                blocks.insert(first_free_space_pos, last_whole_file);
                //print_blocks(&blocks);
                if first_free_space.len - last_whole_file.len > 0 {
                    blocks.insert(
                        first_free_space_pos + 1,
                        DiskEntryWithLen {
                            id: None,
                            entry: DiskEntryType::FreeSpace,
                            len: first_free_space.len - last_whole_file.len,
                        },
                    );
                    //print_blocks(&blocks);
                }
                //println!("AFTER");
                //print_blocks(&blocks);
            }
        } else {
            /*println!(
                "No more free space blocks to move file block id {} at {} with len {}",
                last_whole_file.id.unwrap(),
                last_whole_file_pos,
                last_whole_file.len
            );*/
            ignored_file_ids.insert(last_whole_file.id.unwrap());
            if last_whole_file.id.unwrap() == 1 {
                //println!("No more files to move");
                break;
            }
            continue;
        }
    }

    calc_checksum_whole_files(&blocks)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_checksum_test01() {
        assert_eq!(1928, get_checksum("input/day09_test01.txt"));
    }

    #[test]
    fn test_get_checksum_test02() {
        assert_eq!(60, get_checksum("input/day09_test02.txt"));
    }

    #[test]
    fn test_get_checksum_test03() {
        assert_eq!(513, get_checksum("input/day09_test03.txt"));
    }

    #[test]
    fn test_get_checksum_test04() {
        assert_eq!(57, get_checksum("input/day09_test04.txt"));
    }

    // This test takes a while so ignore in CI

    #[ignore]
    #[test]
    fn test_get_checksum() {
        assert_eq!(6435922584968, get_checksum("input/day09.txt"));
    }

    #[test]
    fn test_get_checksum_whole_files_test01() {
        assert_eq!(2858, get_checksum_whole_files("input/day09_test01.txt"));
    }

    // This test takes a while so ignore in CI

    #[ignore]
    #[test]
    fn test_get_checksum_whole_files() {
        assert_eq!(6469636832766, get_checksum_whole_files("input/day09.txt"));
    }
}
