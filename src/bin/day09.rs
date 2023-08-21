use anyhow::Result;
use aoc_rust_2022::file_to_vec;
use itertools::Itertools;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Position {
    x: i32,
    y: i32,
}
impl Position {
    fn diff(self: &Self, other: &Self) -> Self {
        Position {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

#[derive(Debug, PartialEq)]
struct Move {
    direction: Direction,
    length: usize,
}

#[derive(Debug, PartialEq, Clone)]
struct SingleMove {
    direction: Direction,
}
impl SingleMove {
    fn from_move(m: &Move) -> Vec<Self> {
        vec![
            SingleMove {
                direction: m.direction
            };
            m.length.into()
        ]
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
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

fn part_1(input: &[String]) -> Result<usize, ParseMoveError> {
    let moves = parse_input_to_moves(input)?;
    let positions = move_and_record_positions(moves);
    Ok(positions.into_iter().counts().len())
}

fn parse_input_to_moves(input: &[String]) -> Result<Vec<SingleMove>, ParseMoveError> {
    let moves: Vec<Move> = input
        .iter()
        .map(|line| Move::from_str(line).unwrap())
        .collect();
    let single_moves: Vec<SingleMove> = moves
        .into_iter()
        .map(|m| SingleMove::from_move(&m))
        .flatten()
        .collect();
    Ok(single_moves)
}

fn move_and_record_positions(moves: Vec<SingleMove>) -> Vec<Position> {
    let mut output = Vec::new();
    let mut head_position = Position { x: 0, y: 0 };
    let mut tail_position = Position { x: 0, y: 0 };
    output.push(tail_position.clone());
    for m in moves {
        head_position = move_head(head_position, m);
        tail_position = move_tail(&head_position, &tail_position);
        output.push(tail_position.clone());
    }
    output
}

fn move_tail(head_position: &Position, tail_position: &Position) -> Position {
    let diff = head_position.diff(tail_position);
    let Position { x, y } = diff;
    match (x, y) {
        // X has a large positive difference
        (2.., ..=-2) => Position {
            x: tail_position.x + 1,
            y: tail_position.y - 1,
        },
        (2.., 2..) => Position {
            x: tail_position.x + 1,
            y: tail_position.y + 1,
        },
        (2.., y) => Position {
            x: tail_position.x + 1,
            y: tail_position.y + y,
        },
        // X has a large negative difference
        (..=-2, ..=-2) => Position {
            x: tail_position.x - 1,
            y: tail_position.y - 1,
        },
        (..=-2, 2..) => Position {
            x: tail_position.x - 1,
            y: tail_position.y + 1,
        },
        (..=-2, y) => Position {
            x: tail_position.x - 1,
            y: tail_position.y + y,
        },
        // X difference between -1 and 1
        (x, ..=-2) => Position {
            x: tail_position.x + x,
            y: tail_position.y - 1,
        },
        (x, 2..) => Position {
            x: tail_position.x + x,
            y: tail_position.y + 1,
        },
        (-1..=1, -1..=1) => Position {
            x: tail_position.x,
            y: tail_position.y,
        },
    }
}

fn move_head(head_position: Position, single_move: SingleMove) -> Position {
    match single_move.direction {
        Direction::U => Position {
            x: head_position.x,
            y: head_position.y + 1,
        },
        Direction::D => Position {
            x: head_position.x,
            y: head_position.y - 1,
        },
        Direction::L => Position {
            x: head_position.x - 1,
            y: head_position.y,
        },
        Direction::R => Position {
            x: head_position.x + 1,
            y: head_position.y,
        },
    }
}

fn move_and_record_positions_long_rope(moves: Vec<SingleMove>) -> Vec<Position> {
    let mut output = Vec::new();
    let mut head_position = Position { x: 0, y: 0 };
    let mut tail_positions = vec![Position { x: 0, y: 0 }; 9];
    output.push(tail_positions[8].clone());
    for m in moves {
        head_position = move_head(head_position, m);
        let mut tmp_head = head_position.clone();
        for i in 0..9 {
            tmp_head = move_tail(&tmp_head, &tail_positions[i]);
            tail_positions[i] = tmp_head;
        }
        output.push(tail_positions[8].clone());
    }
    output
}

fn part_2(input: &[String]) -> Result<usize, ParseMoveError> {
    let moves = parse_input_to_moves(input)?;
    let positions: Vec<Position> = move_and_record_positions_long_rope(moves);
    Ok(positions.into_iter().counts().len())
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

        let mut expected = vec![
            SingleMove {
                direction: Direction::R,
            };
            4
        ];
        expected.extend(vec![
            SingleMove {
                direction: Direction::D
            };
            2
        ]);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_move_tail() {
        let head_pos = Position { x: 10, y: 12 };
        let tail_pos = Position { x: 10, y: 10 };
        let result = move_tail(&head_pos, &tail_pos);
        let expected = Position { x: 10, y: 11 };

        assert_eq!(expected, result);
    }

    #[test]
    fn test_move_and_record_positions() {
        let mut input = vec![
            SingleMove {
                direction: Direction::U
            };
            3
        ];
        input.extend(vec![
            SingleMove {
                direction: Direction::R
            };
            3
        ]);
        let result = move_and_record_positions(input);
        let expected = vec![
            Position { x: 0, y: 0 }, // initial
            Position { x: 0, y: 0 }, // U, head at 0,1
            Position { x: 0, y: 1 }, // U, head at 0,2
            Position { x: 0, y: 2 }, // U, head at 0,3
            Position { x: 0, y: 2 }, // R, head at 1,3
            Position { x: 1, y: 3 }, // R, head at 2,3
            Position { x: 2, y: 3 }, // R, head at 3,3
        ];
        assert_eq!(expected, result);
    }

    // Part 2
    #[test]
    fn part_2_sample_input() {
        let filename = "input/day09_sample.txt";
        let input = file_to_vec(filename).unwrap();
        let result = part_2(&input).unwrap();
        assert_eq!(result, 1);
    }

    #[test]
    fn part_2_larger_sample_input() {
        let filename = "input/day09_sample_large.txt";
        let input = file_to_vec(filename).unwrap();
        let result = part_2(&input).unwrap();
        assert_eq!(result, 36);
    }
}
