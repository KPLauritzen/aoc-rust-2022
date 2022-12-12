use aoc_rust_2022::file_to_vec;
use itertools::Itertools;
use std::num::ParseIntError;

const START_OF_PACKET_LENGTH: usize = 4;
const START_OF_MESSAGE_LENGTH: usize = 14;

fn main() {
    let filename = "input/day06.txt";
    let input = file_to_vec(filename).unwrap();
    let part_1_result = part_1(&input).unwrap();
    println!("Part 1: {}", part_1_result);

    let part_2_result = part_2(&input).unwrap();
    println!("Part 2: {}", part_2_result);
}

fn part_1(input: &[String]) -> Result<usize, ParseIntError> {
    let line = input.first().unwrap(); // Input is a single line
    let index = get_first_index_with_all_unique(line, START_OF_PACKET_LENGTH);
    Ok(index)
}

fn get_first_index_with_all_unique(line: &String, n_unique: usize) -> usize {
    for index in n_unique..line.len() {
        let slice = &line[index - n_unique..index];
        if is_all_unique(slice) {
            return index;
        }
    }
    todo!()
}

fn is_all_unique(slice: &str) -> bool {
    let n_chars = slice.len();
    let n_unique_chars = slice.chars().unique().count();
    n_chars == n_unique_chars
}

fn part_2(input:&[String]) -> Result<usize, ParseIntError> {
    let line = input.first().unwrap(); // Input is a single line
    let index = get_first_index_with_all_unique(line, START_OF_MESSAGE_LENGTH);
    Ok(index)
}

#[cfg(test)]
mod tests {
    use super::*;
    // Part 1
    #[test]
    fn part_1_sample_input() {
        let filename = "input/day06_sample.txt";
        let input = file_to_vec(filename).unwrap();
        let result = part_1(&input).unwrap();
        assert_eq!(result, 7);
    }

    // Part 2
    #[test]
    fn part_2_sample_input() {
        let filename = "input/day06_sample.txt";
        let input = file_to_vec(filename).unwrap();
        let result = part_2(&input).unwrap();
        assert_eq!(result, 19);
    }
}
