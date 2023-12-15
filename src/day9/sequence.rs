use crate::string_functions::{all_to_i128, split_and_clean};
use crate::GenericError;

pub struct SequenceExtrapolation {
    sequences: Vec<NumberSequence>,
}

impl SequenceExtrapolation {
    pub fn parse_all(input: &str) -> Result<Vec<SequenceExtrapolation>, GenericError> {
        let mut result = Vec::new();
        for line in input.lines() {
            result.push(SequenceExtrapolation::from_line(line)?);
        }

        return Ok(result);
    }

    fn from_line(input: &str) -> Result<SequenceExtrapolation, GenericError> {
        let mut sequences = Vec::new();
        sequences.push(NumberSequence::parse(input)?);

        while !sequences[sequences.len() - 1].is_constant {
            sequences.push(NumberSequence::difference_sequence(
                &sequences[sequences.len() - 1],
            )?);
        }

        return Ok(SequenceExtrapolation { sequences });
    }

    pub fn value_at_index(&mut self, index: usize) -> i128 {
        if self.sequences.len() < 1 {
            panic!("no sequences in extrapolator found");
        }

        if self.sequences[0].len() > index {
            return self.sequences[0].numbers[index];
        }

        while self.sequences[0].len() <= index {
            let mut next_value = 0i128;
            for i in (0..self.sequences.len()).rev() {
                next_value = self.sequences[i].calculate_and_store_next_value(next_value);
            }
        }

        return self.sequences[0].numbers[index];
    }

    pub fn previous_value(&mut self, number_of_values: i32) -> i128 {
        if self.sequences.len() < 1 {
            panic!("no sequences in extrapolator found");
        }

        for _ in 0..number_of_values {
            let mut previous_value = 0i128;
            for i in (0..self.sequences.len()).rev() {
                previous_value =
                    self.sequences[i].calculate_and_store_previous_value(previous_value);
            }
        }

        return self.sequences[0].numbers[0];
    }

    pub fn len(&self) -> usize {
        return self.sequences[0].len();
    }
}

#[derive(Debug, Eq, PartialEq)]
struct NumberSequence {
    numbers: Vec<i128>,
    is_constant: bool,
}

impl NumberSequence {
    fn parse(input: &str) -> Result<NumberSequence, GenericError> {
        let parts = split_and_clean(input, " ");
        let numbers = all_to_i128(&parts)?;

        if numbers.len() < 1 {
            return Err(GenericError::new("empty number sequence"));
        }

        return Ok(NumberSequence::new(numbers));
    }

    fn difference_sequence(sequence: &NumberSequence) -> Result<NumberSequence, GenericError> {
        if sequence.numbers.len() < 2 {
            return Err(GenericError::new(
                "number sequence must contain at least 2 elements",
            ));
        }

        let mut differences = Vec::new();
        for i in 1..sequence.numbers.len() {
            let difference = &sequence.numbers[i] - &sequence.numbers[i - 1];
            differences.push(difference);
        }

        return Ok(NumberSequence::new(differences));
    }

    fn new(numbers: Vec<i128>) -> NumberSequence {
        if numbers.len() < 1 {
            panic!("empty number sequences are not supported")
        }

        let is_constant = NumberSequence::all_elements_equal(&numbers);
        return NumberSequence {
            numbers,
            is_constant,
        };
    }

    fn all_elements_equal(numbers: &Vec<i128>) -> bool {
        if numbers.len() < 2 {
            return true;
        }

        return numbers.iter().all(|n| *n == numbers[0]);
    }

    pub fn len(&self) -> usize {
        return self.numbers.len();
    }

    pub fn calculate_and_store_next_value(&mut self, difference: i128) -> i128 {
        if self.is_constant && difference != 0 {
            panic!("constant sequences must always have a difference of 0");
        }

        if self.is_constant {
            return self.numbers[0];
        }

        let next_value = last_in(&self.numbers) + difference;
        self.numbers.push(next_value);

        return next_value;
    }

    pub fn calculate_and_store_previous_value(&mut self, difference: i128) -> i128 {
        if self.is_constant && difference != 0 {
            panic!("constant sequences must always have a difference of 0");
        }

        if self.is_constant {
            return self.numbers[0];
        }

        let previous_value = self.numbers[0] - difference;
        self.numbers.insert(0, previous_value);

        return previous_value;
    }
}

fn last_in<T>(items: &[T]) -> &T {
    &items[items.len() - 1]
}

#[cfg(test)]
mod tests {
    use crate::day9::sequence::{NumberSequence, SequenceExtrapolation};

    #[test]
    fn test_sequence_extrapolation_from_input_simple() {
        let input = "0   3   6   9  12  15";
        let actual = SequenceExtrapolation::from_line(&input).unwrap();

        assert_eq!(
            actual.sequences,
            vec![
                NumberSequence::new(vec![0, 3, 6, 9, 12, 15]),
                NumberSequence::new(vec![3, 3, 3, 3, 3]),
            ]
        );
    }

    #[test]
    fn test_sequence_extrapolation_from_input_harder() {
        let input = "10  13  16  21  30  45";
        let actual = SequenceExtrapolation::from_line(&input).unwrap();

        assert_eq!(
            actual.sequences,
            vec![
                NumberSequence::new(vec![10, 13, 16, 21, 30, 45]),
                NumberSequence::new(vec![3, 3, 5, 9, 15]),
                NumberSequence::new(vec![0, 2, 4, 6]),
                NumberSequence::new(vec![2, 2, 2]),
            ]
        );
    }

    #[test]
    fn test_sequence_extrapolation_from_input_with_negatives() {
        let input = "3 2 1 0 -1 -2 -3";
        let actual = SequenceExtrapolation::from_line(&input).unwrap();

        assert_eq!(
            actual.sequences,
            vec![
                NumberSequence::new(vec![3, 2, 1, 0, -1, -2, -3]),
                NumberSequence::new(vec![-1, -1, -1, -1, -1, -1]),
            ]
        )
    }

    #[test]
    fn test_value_at_index_simple() {
        let input = "0   3   6   9  12  15";
        let mut sut = SequenceExtrapolation::from_line(&input).unwrap();

        for i in 0..100 {
            assert_eq!(sut.value_at_index(i), i as i128 * 3);
        }
    }
}
