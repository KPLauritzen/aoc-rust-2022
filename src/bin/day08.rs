use aoc_rust_2022::file_to_vec;
use itertools::Itertools;
use std::num::ParseIntError;

fn main() {
    let filename = "input/day08.txt";
    let input = file_to_vec(filename).unwrap();
    let part_1_result = part_1(&input).unwrap();
    println!("Part 1: {}", part_1_result);

    let part_2_result = part_2(&input).unwrap();
    println!("Part 2: {}", part_2_result);
}

fn part_1(input: &[String]) -> Result<usize, ParseIntError> {
    let tree_heights: Vec<Vec<u32>> = get_tree_heights(input);
    let mut visible: Vec<Vec<bool>> = Vec::new();
    let n_rows = input.len();
    let n_columns = input.first().unwrap().len();
    for _row in 0..n_rows {
        visible.push(vec![true; n_columns])
    }
    // println!("{:?}", tree_heights);
    for (row_idx, row) in visible.iter_mut().enumerate().take(n_rows - 1).skip(1) {
        for col_idx in 1..n_columns - 1 {
            row[col_idx] = is_tree_visible(&tree_heights, row_idx, col_idx);
        }
    }

    // println!("{:?}", visible);
    let count_visible = visible.iter().flatten().filter(|x| **x).count();
    Ok(count_visible)
}

fn get_tree_heights(input: &[String]) -> Vec<Vec<u32>> {
    let mut tree_heights: Vec<Vec<u32>> = Vec::new();
    for line in input {
        tree_heights.push(line.chars().map(|c| c.to_digit(10).unwrap()).collect_vec());
    }
    tree_heights
}
fn is_tree_visible(tree_heights: &Vec<Vec<u32>>, row_idx: usize, col_idx: usize) -> bool {
    let n_rows = tree_heights.len();
    let n_columns = tree_heights.first().unwrap().len();

    let (mut vis_top, mut vis_left, mut vis_right, mut vis_bot) = (true, true, true, true);
    let this_height = tree_heights[row_idx][col_idx];
    // check left. Same row, change cols,
    for cmp_col_idx in 0..col_idx {
        if this_height <= tree_heights[row_idx][cmp_col_idx] {
            vis_left = false;
            break;
        }
    }
    // check top. same col, change rows
    for row in tree_heights.iter().take(row_idx) {
        if this_height <= row[col_idx] {
            vis_top = false;
            break;
        }
    }
    // check right. Same row, change cols,
    for cmp_col_idx in col_idx + 1..n_columns {
        if this_height <= tree_heights[row_idx][cmp_col_idx] {
            vis_right = false;
            break;
        }
    }
    // check top. same col, change rows
    for row in tree_heights.iter().take(n_rows).skip(row_idx + 1) {
        if this_height <= row[col_idx] {
            vis_bot = false;
            break;
        }
    }
    return vec![vis_top, vis_left, vis_right, vis_bot]
        .iter()
        .any(|x| *x);
}
fn part_2(input: &[String]) -> Result<usize, ParseIntError> {
    let tree_heights: Vec<Vec<u32>> = get_tree_heights(input);

    let mut score: Vec<Vec<usize>> = Vec::new();
    let n_rows = input.len();
    let n_columns = input.first().unwrap().len();
    for _row in 0..n_rows {
        score.push(vec![0; n_columns])
    }
    // println!("{:?}", tree_heights);

    for (row_idx, row) in score.iter_mut().enumerate().take(n_rows - 1).skip(1) {
        for col_idx in 1..n_columns - 1 {
            row[col_idx] = get_tree_score(&tree_heights, row_idx, col_idx);
        }
    }
    //println!("{:?}", score);
    let best_score = score.iter().flatten().max().unwrap();
    Ok(*best_score)
}
fn get_tree_score(tree_heights: &Vec<Vec<u32>>, row_idx: usize, col_idx: usize) -> usize {
    let n_rows = tree_heights.len();
    let n_columns = tree_heights.first().unwrap().len();

    let (mut view_top, mut view_left, mut view_right, mut view_bot) = (
        row_idx,
        col_idx,
        n_columns - col_idx - 1,
        n_rows - row_idx - 1,
    );
    let this_height = tree_heights[row_idx][col_idx];

    // println!( "looking at {this_height}");
    // check left. Same row, change cols,
    for cmp_col_idx in (0..col_idx).rev() {
        if this_height <= tree_heights[row_idx][cmp_col_idx] {
            view_left = col_idx - cmp_col_idx;
            break;
        }
    }
    // check top. same col, change rows
    for cmp_row_idx in (0..row_idx).rev() {
        if this_height <= tree_heights[cmp_row_idx][col_idx] {
            view_top = row_idx - cmp_row_idx;
            // println!("Found that row {cmp_row_idx} blocks the view, giving a score of {view_top}");

            break;
        }
    }
    // check right. Same row, change cols,
    for cmp_col_idx in col_idx + 1..n_columns {
        if this_height <= tree_heights[row_idx][cmp_col_idx] {
            view_right = cmp_col_idx - col_idx;
            break;
        }
    }
    // check bot. same col, change rows
    for (cmp_row_idx, row) in tree_heights
        .iter()
        .enumerate()
        .take(n_rows)
        .skip(row_idx + 1)
    {
        if this_height <= row[col_idx] {
            view_bot = cmp_row_idx - row_idx;
            break;
        }
    }
    // println!("top: {view_top}, left {view_left} bot: {view_bot}, right {view_right}");
    view_bot * view_left * view_right * view_top
}

#[cfg(test)]
mod tests {
    use super::*;
    // Part 1
    #[test]
    fn part_1_sample_input() {
        let filename = "input/day08_sample.txt";
        let input = file_to_vec(filename).unwrap();
        let result = part_1(&input).unwrap();
        assert_eq!(result, 21);
    }

    // Part 2
    #[test]
    fn part_2_sample_input() {
        let filename = "input/day08_sample.txt";
        let input = file_to_vec(filename).unwrap();
        let result = part_2(&input).unwrap();
        assert_eq!(result, 8);
    }

    #[test]
    fn test_score_row_3_col_2() {
        let filename = "input/day08_sample.txt";
        let input = file_to_vec(filename).unwrap();

        let tree_heights: Vec<Vec<u32>> = get_tree_heights(&input);

        let score = get_tree_score(&tree_heights, 3, 2);
        assert_eq!(score, 8)
    }

    #[test]
    fn test_score_row_1_col_2() {
        let filename = "input/day08_sample.txt";
        let input = file_to_vec(filename).unwrap();

        let tree_heights: Vec<Vec<u32>> = get_tree_heights(&input);

        let score = get_tree_score(&tree_heights, 1, 2);
        assert_eq!(score, 4)
    }
    #[test]
    fn test_score_row_2_col_2() {
        let filename = "input/day08_sample.txt";
        let input = file_to_vec(filename).unwrap();

        let tree_heights: Vec<Vec<u32>> = get_tree_heights(&input);

        let score = get_tree_score(&tree_heights, 2, 2);
        assert_eq!(score, 1)
    }
}
