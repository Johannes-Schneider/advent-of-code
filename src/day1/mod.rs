use phf::phf_map;
use std::error::Error;
use std::fs;
use substring::Substring;

use advent_of_code::GenericError;

static DIGITS: phf::Map<&'static str, u32> = phf_map! {
    "1" => 1,
    "2" => 2,
    "3" => 3,
    "4" => 4,
    "5" => 5,
    "6" => 6,
    "7" => 7,
    "8" => 8,
    "9" => 9,
    "one" => 1,
    "two" => 2,
    "three" => 3,
    "four" => 4,
    "five" => 5,
    "six" => 6,
    "seven" => 7,
    "eight" => 8,
    "nine" => 9
};

pub fn day1(file_path: &str) -> Result<u32, Box<dyn Error>> {
    let text = fs::read_to_string(file_path)?;
    let digits = find_all_first_and_last_digits(&text);

    if digits.is_none() {
        return Err(Box::new(GenericError::new(
            "not all lines contained (at least) two digits",
        )));
    }

    Ok(digits.unwrap().iter().sum())
}

fn find_all_first_and_last_digits(text: &str) -> Option<Vec<u32>> {
    let mut result: Vec<u32> = Vec::new();

    for (line_number, line) in text.lines().enumerate() {
        let maybe_digits = find_first_and_last_digit(line);
        let maybe_combined_digits = combine_digits(&maybe_digits);

        if maybe_combined_digits.is_none() {
            println!(
                "Line number {} ('{line}') does not contain two digits.",
                line_number + 1
            );
            return None;
        }

        result.push(maybe_combined_digits.unwrap());
    }

    Some(result)
}

fn combine_digits(digits: &Option<(u32, u32)>) -> Option<u32> {
    digits.map(|d| d.0 * 10 + d.1)
}

fn find_first_and_last_digit(input: &str) -> Option<(u32, u32)> {
    let mut first_digit: Option<u32> = None;
    let mut second_digit: Option<u32> = None;

    let mut remaining_input = input;
    while !remaining_input.is_empty() {
        let result = extract_next_digit(remaining_input);
        remaining_input = result.0;

        if result.1.is_none() {
            continue;
        }

        second_digit = result.1;
        if first_digit.is_none() {
            first_digit = result.1;
        }
    }

    if first_digit.is_none() || second_digit.is_none() {
        return None;
    }

    return Some((first_digit.unwrap(), second_digit.unwrap()));
}

fn extract_next_digit(input: &str) -> (&str, Option<u32>) {
    if input.is_empty() {
        return (input, None);
    }

    for key in DIGITS.keys() {
        if !input.starts_with(*key) {
            continue;
        }

        let value = DIGITS.get(key).unwrap();
        let input_left = input.substring(1, input.len());
        return (input_left, Some(*value));
    }

    return (input.substring(1, input.len()), None);
}

#[cfg(test)]
mod tests {
    use super::{
        combine_digits, extract_next_digit, find_all_first_and_last_digits,
        find_first_and_last_digit,
    };

    #[test]
    fn multiple_lines() {
        let input = "\
foo1bar2baz
3bar4
56
oof  7   8 9  har
1
one23fourbar
one
";
        let actual = find_all_first_and_last_digits(input);

        assert_eq!(
            vec_eq(&vec![12, 34, 56, 79, 11, 14, 11], &actual.unwrap()),
            true
        );
    }

    #[test]
    fn example_test() {
        let input = "\
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        let actual = find_all_first_and_last_digits(input);

        assert!(vec_eq(&vec![29, 83, 13, 24, 42, 14, 76], &actual.unwrap()))
    }

    #[test]
    fn written_with_overlap() {
        let input = "\
oneight
1twone";
        let actual = find_all_first_and_last_digits(input);

        assert!(vec_eq(&vec![18, 11], &actual.unwrap()));
    }

    #[test]
    fn extract_next_regular_digit_at_first_index() {
        let input = "1";
        let actual = extract_next_digit(input);

        assert_eq!(("", Some(1)), actual);
    }

    #[test]
    fn extract_next_written_digit_at_first_index() {
        let input = "one";
        let actual = extract_next_digit(input);

        assert_eq!(("ne", Some(1)), actual);
    }

    #[test]
    fn combine_digits_works() {
        let input = Some((1, 1));
        let actual = combine_digits(&input);

        assert_eq!(Some(11), actual);
    }

    #[test]
    fn combine_digits_fails() {
        let input = None;
        let actual = combine_digits(&input);

        assert_eq!(None, actual);
    }

    #[test]
    fn exactly_two_digits() {
        let input = "12";
        let actual = find_first_and_last_digit(input);

        assert_eq!(Some((1, 2)), actual);
    }

    #[test]
    fn two_digits_and_characters() {
        let input = "foo1bar2baz";
        let actual = find_first_and_last_digit(input);

        assert_eq!(Some((1, 2)), actual);
    }

    #[test]
    fn more_than_two_digits_and_characters() {
        let input = "foo1bar2baz3oof";
        let actual = find_first_and_last_digit(input);

        assert_eq!(Some((1, 3)), actual);
    }

    #[test]
    fn written_digits() {
        let input = "fooonebartwobaz";
        let actual = find_first_and_last_digit(input);

        assert_eq!(Some((1, 2)), actual);
    }

    #[test]
    fn written_and_regular_digits() {
        let input = "fooonebar2baz";
        let actual = find_first_and_last_digit(input);

        assert_eq!(Some((1, 2)), actual);
    }

    #[test]
    fn just_one_digit() {
        let input = "foo1bar";
        let actual = find_first_and_last_digit(input);

        assert_eq!(Some((1, 1)), actual);
    }

    #[test]
    fn no_digit() {
        let input = "foo";
        let actual = find_first_and_last_digit(input);

        assert_eq!(None, actual);
    }

    fn vec_eq(first: &Vec<u32>, second: &Vec<u32>) -> bool {
        if first.len() != second.len() {
            return false;
        }

        for (index, first_item) in first.iter().enumerate() {
            let second_item = second.get(index).unwrap();
            if first_item != second_item {
                return false;
            }
        }

        return true;
    }
}
