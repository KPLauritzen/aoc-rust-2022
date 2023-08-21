use anyhow::Result;
use aoc_rust_2022::file_to_vec;
use itertools::enumerate;
use scan_fmt::scan_fmt;

fn main() {
    let filename = "input/day10.txt";
    let input = file_to_vec(filename).unwrap();
    let part_1_result = part_1(&input).unwrap();
    println!("Part 1: {}", part_1_result);

    let part_2_result = part_2(&input).unwrap();
    println!("Part 2: {}", part_2_result);
}

fn part_1(input: &[String]) -> Result<i32> {
    let commands = extract_commands_for_x(input)?;
    let x_val = get_cumulative_values_for_x(commands)?;
    let signal_strengths = get_signal_strengths(x_val)?;
    Ok(signal_strengths.into_iter().sum())
}

fn extract_commands_for_x(input: &[String]) -> Result<Vec<i32>> {
    let mut commands = Vec::new();
    for line in input {
        if line == "noop" {
            commands.push(0);
        } else {
            commands.push(0);
            let x = scan_fmt!(line, "addx {}", i32)?;
            commands.push(x);
        }
    }
    Ok(commands)
}

fn get_cumulative_values_for_x(x_val: Vec<i32>) -> Result<Vec<i32>> {
    let mut prev = 1;
    let mut cumsum = Vec::new();
    for el in x_val {
        cumsum.push(prev + el);
        prev += el;
    }
    Ok(cumsum)
}

fn get_signal_strengths(cumsum: Vec<i32>) -> Result<Vec<i32>> {
    let interesting_indicies = (20..).step_by(40);
    let mut result = Vec::new();
    for idx in interesting_indicies {
        let register = cumsum.get(idx - 2);
        match register {
            Some(val) => result.push(val * i32::try_from(idx)?),
            None => break,
        }
    }
    Ok(result)
}

fn part_2(input: &[String]) -> Result<i32> {
    let commands = extract_commands_for_x(input)?;
    let x_val = get_cumulative_values_for_x(commands)?;
    let lines = get_line_drawing(x_val)?;
    for (idx, line) in lines.into_iter().enumerate() {
        if idx % 40 == 0 {
            println!();
        }
        print!("{}", line);
    }
    Ok(1)
}

fn get_line_drawing(x_val: Vec<i32>) -> Result<Vec<String>> {
    let mut drawing = Vec::new();
    for (cycle, value) in enumerate(x_val) {
        let mod_cycle = cycle % 40;
        if (i32::try_from(mod_cycle)? - value + 1).abs() < 2 {
            drawing.push("#".to_string());
        } else {
            drawing.push(".".to_string())
        }
    }
    Ok(drawing)
}

#[cfg(test)]
mod tests {
    use super::*;
    // Part 1
    #[test]
    fn part_1_sample_input() {
        let filename = "input/day10_sample.txt";
        let input = file_to_vec(filename).unwrap();
        let result = part_1(&input).unwrap();
        assert_eq!(result, 13140);
    }

    #[test]
    fn test_extract_commands_for_x() {
        let input = vec![
            "addx 10".to_string(),
            "addx -2".to_string(),
            "noop".to_string(),
        ];
        let result = extract_commands_for_x(&input).unwrap();
        let expected = vec![0, 10, 0, -2, 0];

        assert_eq!(result, expected);
    }

    // Part 2
    #[test]
    fn part_2_sample_input() {
        let filename = "input/day10_sample.txt";
        let input = file_to_vec(filename).unwrap();
        let result = part_2(&input).unwrap();
        assert_eq!(result, 1);
    }
}
