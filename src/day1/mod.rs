use std::error::Error;
use std::fs;

use advent_of_code::GenericError;

pub fn day1(file_path: &str) -> Result<u32, Box<dyn Error>> {
    let text = fs::read_to_string(file_path)?;
    let digits = find_all_first_and_last_digits(&text);

    if digits.is_none() {
        return Err(Box::new(GenericError::new("not all lines contained (at least) two digits")) as Box<dyn Error>);
    }

    Ok(digits.unwrap().iter().sum())
}

fn find_all_first_and_last_digits(text: &str) -> Option<Vec<u32>> {
    let mut result: Vec<u32> = Vec::new();

    for (line_number, line) in text.lines().enumerate() {
        let maybe_digits = find_first_and_last_digit(line);
        let maybe_combined_digits = combine_digits(&maybe_digits);

        if maybe_combined_digits.is_none() {
            println!("Line number {} ('{line}') does not contain two digits.", line_number + 1);
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

    for character in input.chars() {
        let maybe_digit = character.to_digit(10);
        if maybe_digit.is_some() {
            second_digit = maybe_digit;

            if first_digit == None {
                first_digit = maybe_digit;
            }
        }
    }

    if first_digit == None || second_digit == None {
        None
    } else {
        Some((first_digit.unwrap(), second_digit.unwrap()))
    }
}

#[cfg(test)]
mod tests {
    use super::{find_all_first_and_last_digits, find_first_and_last_digit, combine_digits};

    #[test]
    fn multiple_lines() {
        let input = "\
foo1bar2baz
3bar4
56
oof  7   8 9  har
1
";
        let actual = find_all_first_and_last_digits(input);

        assert_eq!(vec_eq(&vec![12, 34, 56, 79, 11], &actual.unwrap()), true);
    }

    #[test]
    fn combine_digits_works()
    {
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