// https://adventofcode.com/2022/day/1

use std::num::ParseIntError;
use aoc_rust_2022::{file_to_vec};



fn main() {
    let filename = "input/day01.txt";
    let input = file_to_vec(filename).unwrap();
    let part_1_result = part_1(&input).unwrap();
    println!("Part 1: {}", part_1_result);

    let part_2_result = part_2(&input).unwrap();
    println!("Part 2: {}", part_2_result);

}

fn get_total_calories(input: &Vec<String>) -> Result<Vec<i32>, ParseIntError> {
    let mut elves: Vec<i32> = Vec::new();
    let mut cal_count = 0;
    for line in input {
        if let Ok(calories) = line.parse::<i32>() {
            cal_count += calories;
        } else {
            elves.push(cal_count);
            cal_count = 0
        }
    }
    
    // Hack to get the last line
    if cal_count != 0 {
        elves.push(cal_count);
    };
    Ok(elves)
}


fn part_1(input: &Vec<String>) -> Result<i32, ParseIntError> {
    let elves = get_total_calories(input)?;

    Ok(*elves.iter().max().unwrap())
}

fn part_2(input: &Vec<String>) -> Result<i32, ParseIntError> {
    let mut elves = get_total_calories(input)?;
    elves.sort();
    elves.reverse();
    let cals = elves.iter().take(3).sum();

    Ok(cals)
}
#[cfg(test)]
mod tests {
    use super::*;
    // Part 1
    #[test]
    fn part_1_sample_input() {
        let filename = "input/day01_sample.txt";
        let input = file_to_vec(filename).unwrap();
        let result = part_1(&input).unwrap();
        assert_eq!(result, 24000);
    }

        // Part 2
        #[test]
        fn part_2_sample_input() {
            let filename = "input/day01_sample.txt";
            let input = file_to_vec(filename).unwrap();
            let result = part_2(&input).unwrap();
            assert_eq!(result, 45000);
        }
}
