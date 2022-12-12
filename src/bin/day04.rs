// https://adventofcode.com/2022/day/4

use aoc_rust_2022::file_to_vec;
use std::num::ParseIntError;
use scan_fmt::scan_fmt;

fn main() {
    let filename = "input/day04.txt";
    let input = file_to_vec(filename).unwrap();
    let part_1_result = part_1(&input).unwrap();
    println!("Part 1: {}", part_1_result);

    let part_2_result = part_2(&input).unwrap();
    println!("Part 2: {}", part_2_result);
}

// How many pairs, where one fully contains the other
fn part_1(input: &[String]) -> Result<usize, ParseIntError> {
    let count: usize = input
        .iter()
        // Here I'm parsing every line with a new crate I found, scan_fmt
        .map(|line| {
            scan_fmt!(
                line,              // input
                "{d}-{d},{d}-{d}", // expected format if the input
                i32,               // Type of each element of the input
                i32,               // Same
                i32,               // Same
                i32                // Same
            )
            .unwrap()
        })
        .filter(|(a, b, c, d)| is_fully_contained(a, b, c, d))
        .count();

    Ok(count)
}

fn is_fully_contained(
    range_1_start: &i32,
    range_1_end: &i32,
    range_2_start: &i32,
    range_2_end: &i32,
) -> bool {
    (range_1_start <= range_2_start && range_1_end >= range_2_end) // range_1 contains range_2
        || (range_2_start <= range_1_start) && (range_2_end >= range_1_end) // range_2 contains range_1
}

// How many pairs where there is any overlap
fn part_2(input: &[String]) -> Result<usize, ParseIntError> {
    let count: usize = input
        .iter()
        // Here I'm parsing every line with a new crate I found, scan_fmt
        .map(|line| {
            scan_fmt!(
                line,              // input
                "{d}-{d},{d}-{d}", // expected format if the input
                i32,               // Type of each element of the input
                i32,               // Same
                i32,               // Same
                i32                // Same
            )
            .unwrap()
        })
        .filter(|(a, b, c, d)| has_any_overlap(a, b, c, d))
        .count();

    Ok(count)
}

fn has_any_overlap(
    range_1_start: &i32,
    range_1_end: &i32,
    range_2_start: &i32,
    range_2_end: &i32,
) -> bool {
    (range_1_start <= range_2_start) && (range_1_end >= range_2_start)
        || (range_2_start <= range_1_start) && (range_2_end >= range_1_start)
}

#[cfg(test)]
mod tests {
    use super::*;
    // Part 1
    #[test]
    fn part_1_sample_input() {
        let filename = "input/day04_sample.txt";
        let input = file_to_vec(filename).unwrap();
        let result = part_1(&input).unwrap();
        assert_eq!(result, 2);
    }
    // Part 2
    #[test]
    fn part_2_sample_input() {
        let filename = "input/day04_sample.txt";
        let input = file_to_vec(filename).unwrap();
        let result = part_2(&input).unwrap();
        assert_eq!(result, 4);
    }

    #[test]
    fn test_has_any_overlap() {
        let actual = has_any_overlap(&1, &2, &3, &4);
        let expected = false;
        assert_eq!(actual, expected);

        let actual = has_any_overlap(&1, &3, &3, &4);
        let expected = true;
        assert_eq!(actual, expected)
    }
}
