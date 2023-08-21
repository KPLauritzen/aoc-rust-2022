use aoc_rust_2022::file_to_vec;
use std::num::ParseIntError;

fn main() {
    let filename = "input/dayXX.txt";
    let input = file_to_vec(filename).unwrap();
    let part_1_result = part_1(&input).unwrap();
    println!("Part 1: {}", part_1_result);

    let part_2_result = part_2(&input).unwrap();
    println!("Part 2: {}", part_2_result);
}

fn part_1(input: &[String]) -> Result<i32, ParseIntError> {
    todo!()
}

fn part_2(input: &[String]) -> Result<i32, ParseIntError> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    // Part 1
    #[test]
    fn part_1_sample_input() {
        let filename = "input/dayXX_sample.txt";
        let input = file_to_vec(filename).unwrap();
        let result = part_1(&input).unwrap();
        assert_eq!(result, 24000);
    }

    // Part 2
    #[test]
    fn part_2_sample_input() {
        let filename = "input/dayXX_sample.txt";
        let input = file_to_vec(filename).unwrap();
        let result = part_2(&input).unwrap();
        assert_eq!(result, 45000);
    }
}
