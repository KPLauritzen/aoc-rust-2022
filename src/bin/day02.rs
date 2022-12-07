// https://adventofcode.com/2022/day/1

use std::num::ParseIntError;
use aoc_rust_2022::{file_to_vec};
use std::cmp::Ordering;

#[derive(PartialEq, Debug, Clone)]
enum RPS {
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

impl RPS {
    fn get_outcome_against(&self, other: RPS) -> Outcome {
        match self {
            RPS::Rock => {
                match other {
                    RPS::Rock => Outcome::Draw,
                    RPS::Paper => Outcome::Loss,
                    RPS::Scissors => Outcome::Win,
                }
            }
            RPS::Paper => {
                match other {
                    RPS::Rock => Outcome::Win,
                    RPS::Paper => Outcome::Draw,
                    RPS::Scissors => Outcome::Loss,
                }
            }
            RPS::Scissors => {
                match other {
                    RPS::Rock => Outcome::Loss,
                    RPS::Paper => Outcome::Win,
                    RPS::Scissors => Outcome::Draw,
                }
            }
        }
    }

    fn get_needed_move(&self, outcome: Outcome) -> RPS {
        match (self, outcome) {
            (RPS::Rock, Outcome::Win) => RPS::Paper,
            (RPS::Rock, Outcome::Loss) => RPS::Scissors,
            (RPS::Rock, Outcome::Draw) => RPS::Rock,

            (RPS::Paper, Outcome::Win) => RPS::Scissors,
            (RPS::Paper, Outcome::Loss) => RPS::Rock,
            (RPS::Paper, Outcome::Draw) => RPS::Paper,

            (RPS::Scissors, Outcome::Win) => RPS::Rock,
            (RPS::Scissors, Outcome::Loss) => RPS::Paper,
            (RPS::Scissors, Outcome::Draw) => RPS::Scissors,
        }
    }
}




fn match_score(rps: RPS, outcome: Outcome) -> i32 {
    (rps as i32) + (outcome.clone() as i32)
}



fn main() {
    let filename = "input/day02.txt";
    let input = file_to_vec(filename).unwrap();
    let part_1_result = part_1(&input).unwrap();
    println!("Part 1: {}", part_1_result);

    let part_2_result = part_2(&input).unwrap();
    println!("Part 2: {}", part_2_result);

}

fn parse_moves(input: &Vec<String>) -> (Vec<RPS>, Vec<RPS>) {
    let mut opp_moves: Vec<RPS> = Vec::new();
    let mut my_moves: Vec<RPS> = Vec::new();
    for line in input {
        let (opp_move, my_move) = line.split_once(' ').unwrap();
        match opp_move {
            "A" => opp_moves.push(RPS::Rock),
            "B" => opp_moves.push(RPS::Paper),
            "C" => opp_moves.push(RPS::Scissors),

            _ => todo!(),
        }
        match my_move {
            "X" => my_moves.push(RPS::Rock),
            "Y" => my_moves.push(RPS::Paper),
            "Z" => my_moves.push(RPS::Scissors),

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

fn parse_moves_and_outcomes(input: &Vec<String>) -> (Vec<RPS>, Vec<Outcome>) {
    let mut opp_moves: Vec<RPS> = Vec::new();
    let mut outcomes: Vec<Outcome> = Vec::new();
    for line in input {
        let (opp_move, my_move) = line.split_once(' ').unwrap();
        match opp_move {
            "A" => opp_moves.push(RPS::Rock),
            "B" => opp_moves.push(RPS::Paper),
            "C" => opp_moves.push(RPS::Scissors),

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
        let rps = RPS::Paper;
        let outcome = Outcome::Win;
        let actual_score = match_score(rps, outcome);
        assert_eq!(actual_score, 8)
    }

    #[test]
    fn test_match_score_2() {
        let rps = RPS::Rock;
        let outcome = Outcome::Loss;
        let actual_score = match_score(rps, outcome);
        assert_eq!(actual_score, 1)
    }

    #[test]
    fn test_match_score_3() {
        let rps = RPS::Scissors;
        let outcome = Outcome::Draw;
        let actual_score = match_score(rps, outcome);
        assert_eq!(actual_score, 6)
    }

    #[test]
    fn test_rps_ordering() {
        let expected_outcome = Outcome::Loss;
        let actual_outcome = RPS::Rock.get_outcome_against(RPS::Paper);
        assert_eq!(expected_outcome, actual_outcome);

        let expected_outcome = Outcome::Win;
        let actual_outcome = RPS::Paper.get_outcome_against(RPS::Rock);
        assert_eq!(expected_outcome, actual_outcome);

        let expected_outcome = Outcome::Draw;
        let actual_outcome = RPS::Scissors.get_outcome_against(RPS::Scissors);
        assert_eq!(expected_outcome, actual_outcome);
    }

    #[test]
    fn test_parsing_moves() {
        let filename = "input/day02_sample.txt";
        let input = file_to_vec(filename).unwrap();
        let (opp_moves, my_moves) = parse_moves(&input);
        assert_eq!(opp_moves, vec![RPS::Rock, RPS::Paper, RPS::Scissors]);
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
