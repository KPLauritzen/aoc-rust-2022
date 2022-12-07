use std::fs;
use std::io;
use std::io::{BufRead, BufReader};
use std::num::ParseIntError;

/// Read the file at `filename` and returns each line as a `String` in a `Vec`
pub fn file_to_vec(filename: &str) -> io::Result<Vec<String>> {
    let file_in = fs::File::open(filename)?;
    let file_reader = BufReader::new(file_in);
    Ok(file_reader.lines().filter_map(io::Result::ok).collect())
}

pub fn string_to_int(input: &Vec<String>) -> Result<Vec<i32>, ParseIntError> {
    // Using an iterator, convert each string to an integer
    let numbers: Vec<i32> = input
        .iter()
        .map(|s| s.parse::<i32>())
        .collect::<Result<Vec<i32>, ParseIntError>>()?;
    Ok(numbers)
}

#[cfg(test)]
mod tests {
    use std::{num::IntErrorKind, vec};

    use super::*;
    #[test]
    fn can_read_input() {
        let _vec = file_to_vec("input/day01_sample.txt").unwrap();
    }

    #[test]
    #[should_panic]
    fn panics_on_bad_filename() {
        let _vec = file_to_vec("input/not-a-real-file.txt").unwrap();
    }
    #[test]
    fn parse_string_to_int() {
        let vec_str = vec!["1".to_owned()];
        let vec_int = string_to_int(&vec_str);
        assert_eq!(vec_int, Ok(vec![1]));
    }
    #[test]
    fn parse_string_to_int_fails_for_nonint() {
        let vec_str = vec!["not a number".to_owned()];
        let vec_int = string_to_int(&vec_str);
        assert!(vec_int.is_err());
    }
    #[test]
    fn parse_string_to_int_fails_with_parseinterror_for_nonint() {
        let vec_str = vec!["not a number".to_owned()];

        let expected_error_kind = IntErrorKind::InvalidDigit;
        let actual_error_kind = string_to_int(&vec_str).unwrap_err().kind().clone();

        assert_eq!(actual_error_kind, expected_error_kind);
    }
}
