// https://adventofcode.com/2022/day/3

use std::num::ParseIntError;
use aoc_rust_2022::{file_to_vec};



fn main() {
    let filename = "input/day03.txt";
    let input = file_to_vec(filename).unwrap();
    let part_1_result = part_1(&input).unwrap();
    println!("Part 1: {}", part_1_result);

    let part_2_result = part_2(&input).unwrap();
    println!("Part 2: {}", part_2_result);

}


fn part_1(input: &Vec<String>) -> Result<i32, ParseIntError> {
    let mut score = 0;
    for line in input {
        let (left, right) = split_line_at_middle(&line);
        let overlap = find_overlapping_type(&left, &right);
        let priority = get_priority(overlap);
        score += priority;
    };
    Ok(score)
}

fn split_line_at_middle(line: &str) -> (&str, &str) {
    let split_idx = line.len() / 2;
    let (left, right) = line.split_at(split_idx);
    (left, right)
}

fn find_overlapping_type(left: &str, right: &str) -> char {
    for element in left.chars() {
        if right.contains(element) {
            return element
        };
    };
    panic!("Could not find any overlap");
}

fn get_priority(overlap: char) -> i32 {
    let alphabet = " abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let priority = alphabet.find(overlap).unwrap();
    priority as i32
}

fn part_2(input: &Vec<String>) -> Result<i32, ParseIntError> {
    let iter = input.iter().zip(input.iter().skip(1)).zip(input.iter().skip(2));
    let mut score = 0;
    for ((line1, line2), line3) in iter.step_by(3) {
        let overlap_1_2: String = line1.chars().filter(|x| line2.contains(*x)).collect();
        let overlap_1_2_3 = find_overlapping_type(&overlap_1_2, line3);
        let priority = get_priority(overlap_1_2_3);
                score += priority;

    };
    Ok(score)}

#[cfg(test)]
mod tests {
    use super::*;
    // Part 1
    #[test]
    fn part_1_sample_input() {
        let filename = "input/day03_sample.txt";
        let input = file_to_vec(filename).unwrap();
        let result = part_1(&input).unwrap();
        assert_eq!(result, 157);
    }

    // Part 2
    #[test]
    fn part_2_sample_input() {
        let filename = "input/day03_sample.txt";
        let input = file_to_vec(filename).unwrap();
        let result = part_2(&input).unwrap();
        assert_eq!(result, 70);
    }
}
