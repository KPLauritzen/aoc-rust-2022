// https://adventofcode.com/2022/day/5

use aoc_rust_2022::file_to_vec;
use std::num::ParseIntError;
use scan_fmt::scan_fmt;
const N_CHARS_PER_STACK: usize = 4;


fn main() {
    let filename = "input/day05.txt";
    let input = file_to_vec(filename).unwrap();
    let part_1_result = part_1(&input).unwrap();
    println!("Part 1: {}", part_1_result);

    let part_2_result = part_2(&input).unwrap();
    println!("Part 2: {}", part_2_result);
}

fn part_1(input: &[String]) -> Result<String, ParseIntError> {
    let n_stacks = (input[0].len() + 1) / N_CHARS_PER_STACK;
    let highest_stack = get_highest_stack(input);
    let mut stacks: Vec<Vec<char>> = get_stacks(input, n_stacks, highest_stack);
    stacks = move_supply_crates(input, stacks, highest_stack);
    let top_of_stacks = get_top_of_stacks(stacks);
    Ok(top_of_stacks)
}

fn get_highest_stack(input: &[String]) -> usize {
    // Get the index of the first line that does not contain a '['
    input.iter().position(|line| !line.contains('[')).unwrap()
}

fn get_stacks(input: &[String], n_stacks: usize, highest_stack: usize) -> Vec<Vec<char>> {
    let mut stacks: Vec<Vec<char>> = Vec::new();
    // create empty stacks
    for _ in 0..n_stacks {
        stacks.push(Vec::new());
    }
    for line in input.iter().take(highest_stack) {
        // Go through every 4th char in the line, skipping the first char.
        for (stack_idx, supply_crate) in line.chars().skip(1).step_by(N_CHARS_PER_STACK).enumerate()
        {
            if supply_crate != ' ' {
                stacks[stack_idx].push(supply_crate);
            }
        }
    }
    for stack in stacks.iter_mut() {
        stack.reverse()
    }
    stacks
}

fn move_supply_crates(
    input: &[String],
    mut stacks: Vec<Vec<char>>,
    highest_stack: usize,
) -> Vec<Vec<char>> {
    // After the initial supply crate state, there are two lines before the move commands
    for line in input.iter().skip(highest_stack + 2) {
        // Parse each line of commands according to the given pattern
        let (n_moves, from_stack, to_stack) =
            scan_fmt!(line, "move {d} from {d} to {d}", usize, usize, usize).unwrap();
        for _ in 0..n_moves {
            // indexing is off-by-one
            let supply_crate = stacks[from_stack - 1].pop().unwrap();
            stacks[to_stack - 1].push(supply_crate);
        }
    }
    stacks
}

fn get_top_of_stacks(stacks: Vec<Vec<char>>) -> String {
    let mut out = String::new();
    for stack in stacks {
        out.push(*stack.last().unwrap())
    }
    out
}

fn part_2(input: &[String]) -> Result<String, ParseIntError> {
    let n_stacks = (input[0].len() + 1) / N_CHARS_PER_STACK;
    let highest_stack = get_highest_stack(input);
    let mut stacks: Vec<Vec<char>> = get_stacks(input, n_stacks, highest_stack);
    stacks = move_supply_crates_9001(input, stacks, highest_stack);
    let top_of_stacks = get_top_of_stacks(stacks);
    Ok(top_of_stacks)
}

fn move_supply_crates_9001(
    input: &[String],
    mut stacks: Vec<Vec<char>>,
    highest_stack: usize,
) -> Vec<Vec<char>> {
    // After the initial supply crate state, there are two lines before the move commands
    for line in input.iter().skip(highest_stack + 2) {
        // Parse each line of commands according to the given pattern
        let (n_moves, from_stack, to_stack) =
            scan_fmt!(line, "move {d} from {d} to {d}", usize, usize, usize).unwrap();
        // We will simulate the CrateMover9001 by moving crates to a temporary stack one at a time, reversing the order of the temp stack
        // adding it to the target stack.
        let mut temp_stack = Vec::new();
        for _ in 0..n_moves {
            // indexing is off-by-one
            let supply_crate = stacks[from_stack - 1].pop().unwrap();
            temp_stack.push(supply_crate);
        }
        temp_stack.reverse();
        stacks[to_stack - 1].append(&mut temp_stack);
    }
    stacks
}

#[cfg(test)]
mod tests {
    use super::*;
    // Part 1
    #[test]
    fn part_1_sample_input() {
        let filename = "input/day05_sample.txt";
        let input = file_to_vec(filename).unwrap();
        let result = part_1(&input).unwrap();
        assert_eq!(result, "CMZ");
    }

    #[test]
    fn parse_stacks() {
        let filename = "input/day05_sample.txt";
        let input = file_to_vec(filename).unwrap();
        let actual = get_stacks(&input, 3, 3);
        let expected = vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']];
        assert_eq!(actual, expected);
    }

    // Part 2
    #[test]
    fn part_2_sample_input() {
        let filename = "input/day05_sample.txt";
        let input = file_to_vec(filename).unwrap();
        let result = part_2(&input).unwrap();
        assert_eq!(result, "MCD");
    }
}
