// https://adventofcode.com/2022/day/2

use aoc_rust_2022::file_to_vec;
use std::num::ParseIntError;

#[derive(PartialEq, Debug, Clone)]
enum Throw {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}
#[derive(PartialEq, Debug, Clone)]
enum Outcome {
    Win = 6,
    Loss = 0,
    Draw = 3,
}

impl Throw {
    fn get_outcome_against(&self, other: Throw) -> Outcome {
        match self {
            Throw::Rock => match other {
                Throw::Rock => Outcome::Draw,
                Throw::Paper => Outcome::Loss,
                Throw::Scissors => Outcome::Win,
            },
            Throw::Paper => match other {
                Throw::Rock => Outcome::Win,
                Throw::Paper => Outcome::Draw,
                Throw::Scissors => Outcome::Loss,
            },
            Throw::Scissors => match other {
                Throw::Rock => Outcome::Loss,
                Throw::Paper => Outcome::Win,
                Throw::Scissors => Outcome::Draw,
            },
        }
    }

    fn get_needed_move(&self, outcome: Outcome) -> Throw {
        match (self, outcome) {
            (Throw::Rock, Outcome::Win) => Throw::Paper,
            (Throw::Rock, Outcome::Loss) => Throw::Scissors,
            (Throw::Rock, Outcome::Draw) => Throw::Rock,

            (Throw::Paper, Outcome::Win) => Throw::Scissors,
            (Throw::Paper, Outcome::Loss) => Throw::Rock,
            (Throw::Paper, Outcome::Draw) => Throw::Paper,

            (Throw::Scissors, Outcome::Win) => Throw::Rock,
            (Throw::Scissors, Outcome::Loss) => Throw::Paper,
            (Throw::Scissors, Outcome::Draw) => Throw::Scissors,
        }
    }
}

fn match_score(rps: Throw, outcome: Outcome) -> i32 {
    (rps as i32) + (outcome as i32)
}

fn main() {
    let filename = "input/day02.txt";
    let input = file_to_vec(filename).unwrap();
    let part_1_result = part_1(&input).unwrap();
    println!("Part 1: {}", part_1_result);

    let part_2_result = part_2(&input).unwrap();
    println!("Part 2: {}", part_2_result);
}

fn parse_moves(input: &Vec<String>) -> (Vec<Throw>, Vec<Throw>) {
    let mut opp_moves: Vec<Throw> = Vec::new();
    let mut my_moves: Vec<Throw> = Vec::new();
    for line in input {
        let (opp_move, my_move) = line.split_once(' ').unwrap();
        match opp_move {
            "A" => opp_moves.push(Throw::Rock),
            "B" => opp_moves.push(Throw::Paper),
            "C" => opp_moves.push(Throw::Scissors),

            _ => todo!(),
        }
        match my_move {
            "X" => my_moves.push(Throw::Rock),
            "Y" => my_moves.push(Throw::Paper),
            "Z" => my_moves.push(Throw::Scissors),

            _ => todo!(),
        }
    }
    (opp_moves, my_moves)
}

fn part_1(input: &Vec<String>) -> Result<i32, ParseIntError> {
    let mut running_score = 0;
    let (opp_moves, my_moves) = parse_moves(input);
    for (opp_move, my_move) in opp_moves.into_iter().zip(my_moves.into_iter()) {
        let outcome = my_move.get_outcome_against(opp_move);
        let score = match_score(my_move, outcome);
        running_score += score;
    }
    Ok(running_score)
}

fn parse_moves_and_outcomes(input: &Vec<String>) -> (Vec<Throw>, Vec<Outcome>) {
    let mut opp_moves: Vec<Throw> = Vec::new();
    let mut outcomes: Vec<Outcome> = Vec::new();
    for line in input {
        let (opp_move, my_move) = line.split_once(' ').unwrap();
        match opp_move {
            "A" => opp_moves.push(Throw::Rock),
            "B" => opp_moves.push(Throw::Paper),
            "C" => opp_moves.push(Throw::Scissors),

            _ => todo!(),
        }
        match my_move {
            "X" => outcomes.push(Outcome::Loss),
            "Y" => outcomes.push(Outcome::Draw),
            "Z" => outcomes.push(Outcome::Win),

            _ => todo!(),
        }
    }
    (opp_moves, outcomes)
}

fn part_2(input: &Vec<String>) -> Result<i32, ParseIntError> {
    let mut running_score = 0;
    let (opp_moves, desired_outcomes) = parse_moves_and_outcomes(input);
    for (opp_move, desired_outcome) in opp_moves.into_iter().zip(desired_outcomes.into_iter()) {
        // println!("Input: {:?}, {:?} ", opp_move, desired_outcome);

        let my_move = opp_move.get_needed_move(desired_outcome.clone());
        // println!("My move: {:?}", my_move);

        let score = match_score(my_move, desired_outcome);

        // println!("Score: {}", score);
        running_score += score;
    }
    Ok(running_score)
}

#[cfg(test)]
mod tests {
    use super::*;
    // Part 1
    #[test]
    fn part_1_sample_input() {
        let filename = "input/day02_sample.txt";
        let input = file_to_vec(filename).unwrap();
        let result = part_1(&input).unwrap();
        assert_eq!(result, 15);
    }

    #[test]
    fn test_match_score_1() {
        let rps = Throw::Paper;
        let outcome = Outcome::Win;
        let actual_score = match_score(rps, outcome);
        assert_eq!(actual_score, 8)
    }

    #[test]
    fn test_match_score_2() {
        let rps = Throw::Rock;
        let outcome = Outcome::Loss;
        let actual_score = match_score(rps, outcome);
        assert_eq!(actual_score, 1)
    }

    #[test]
    fn test_match_score_3() {
        let rps = Throw::Scissors;
        let outcome = Outcome::Draw;
        let actual_score = match_score(rps, outcome);
        assert_eq!(actual_score, 6)
    }

    #[test]
    fn test_rps_ordering() {
        let expected_outcome = Outcome::Loss;
        let actual_outcome = Throw::Rock.get_outcome_against(Throw::Paper);
        assert_eq!(expected_outcome, actual_outcome);

        let expected_outcome = Outcome::Win;
        let actual_outcome = Throw::Paper.get_outcome_against(Throw::Rock);
        assert_eq!(expected_outcome, actual_outcome);

        let expected_outcome = Outcome::Draw;
        let actual_outcome = Throw::Scissors.get_outcome_against(Throw::Scissors);
        assert_eq!(expected_outcome, actual_outcome);
    }

    #[test]
    fn test_parsing_moves() {
        let filename = "input/day02_sample.txt";
        let input = file_to_vec(filename).unwrap();
        let (opp_moves, _my_moves) = parse_moves(&input);
        assert_eq!(opp_moves, vec![Throw::Rock, Throw::Paper, Throw::Scissors]);
    }

    // Part 2
    #[test]
    fn part_2_sample_input() {
        let filename = "input/day02_sample.txt";
        let input = file_to_vec(filename).unwrap();
        let result = part_2(&input).unwrap();
        assert_eq!(result, 12);
    }
}
