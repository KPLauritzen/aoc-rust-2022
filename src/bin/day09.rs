use aoc_rust_2022::file_to_vec;
use std::fmt::Error;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Debug, PartialEq)]
struct Move {
    direction: Direction,
    length: i32,
}

#[derive(Debug, PartialEq)]
enum Direction {
    U,
    D,
    L,
    R,
}
#[derive(Debug)]
struct ParseMoveError;
impl FromStr for Move {
    type Err = ParseMoveError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (d, l) = s.split_once(" ").ok_or(ParseMoveError)?;

        let direction = match d {
            "U" => Direction::U,
            "D" => Direction::D,
            "L" => Direction::L,
            "R" => Direction::R,
            _ => return Err(ParseMoveError),
        };
        let length = l.parse().map_err(|_| ParseMoveError)?;
        Ok(Move {
            direction: direction,
            length: length,
        })
    }
}
fn main() {
    let filename = "input/day09.txt";
    let input = file_to_vec(filename).unwrap();
    let part_1_result = part_1(&input).unwrap();
    println!("Part 1: {}", part_1_result);

    let part_2_result = part_2(&input).unwrap();
    println!("Part 2: {}", part_2_result);
}

fn part_1(input: &[String]) -> Result<i32, ParseMoveError> {
    let moves = parse_input_to_moves(input)?;
    let positions = move_and_record_positions(moves);
    todo!()
}

fn parse_input_to_moves(input: &[String]) -> Result<Vec<Move>, ParseMoveError> {
    input.iter().map(|line| Move::from_str(line)).collect()
}

fn move_and_record_positions(moves: Vec<Move>) -> Vec<Position> {
    let mut output = Vec::new();
    let mut position = Position{ x: 0, y: 0 };
    output.push(position.clone());
    for m in moves {
        position = do_single_move(&position, m);
        output.push(position.clone());
    };
    output
}

fn do_single_move(start_position: &Position, single_move: Move) -> Position {
    
    match single_move.direction {
        Direction::U => Position { x: start_position.x, y: start_position.y + single_move.length },
        Direction::D => Position { x: start_position.x, y: start_position.y - single_move.length },
        Direction::L => Position { x: start_position.x - single_move.length, y: start_position.y },
        Direction::R => Position { x: start_position.x + single_move.length, y: start_position.y },
    }
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
        let filename = "input/day09_sample.txt";
        let input = file_to_vec(filename).unwrap();
        let result = part_1(&input).unwrap();
        assert_eq!(result, 13);
    }

    #[test]
    fn test_parse_input_to_moves() {
        let input = vec!["R 4".to_owned(), "D 2".to_owned()];
        let result = parse_input_to_moves(&input).unwrap();
        let expected = vec![
            Move {
                direction: Direction::R,
                length: 4,
            },
            Move {
                direction: Direction::D,
                length: 2,
            },
        ];
        assert_eq!(result, expected);
    }
    #[test]
    fn test_move_and_record_positions() {
        let input = vec![
            Move {
                direction: Direction::R,
                length: 4,
            },
            Move {
                direction: Direction::D,
                length: 2,
            },
        ];
        let result = move_and_record_positions(input);
        let expected = vec![
            Position{x:0,y:0},
            Position{x:4,y:0},
            Position{x:4,y:-2},
        ];
        assert_eq!(result, expected);
    }

    // Part 2
    #[test]
    fn part_2_sample_input() {
        let filename = "input/day09_sample.txt";
        let input = file_to_vec(filename).unwrap();
        let result = part_2(&input).unwrap();
        assert_eq!(result, 45000);
    }
}
