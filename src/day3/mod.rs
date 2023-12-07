use std::error::Error;
use std::fs;
use std::ops::Range;

use advent_of_code::GenericError;

static DOT: u8 = 46;
static NUMBER_0: u8 = 48;
static NUMBER_9: u8 = NUMBER_0 + 9;

#[derive(Debug, PartialEq)]
enum Symbol {
    Number(u8),
    Dot,
    Other,
}

impl Symbol {
    fn parse_all(input: &[u8]) -> Vec<Symbol> {
        let mut result: Vec<Symbol> = Vec::new();
        for byte in input {
            result.push(Symbol::parse(*byte));
        }

        return result;
    }

    fn parse(input: u8) -> Symbol {
        if input == DOT {
            return Symbol::Dot;
        }

        if input >= NUMBER_0 && input <= NUMBER_9 {
            return Symbol::Number(input - NUMBER_0);
        }

        return Symbol::Other;
    }
}

struct EngineSchema {
    dimension_x: usize,
    dimension_y: usize,
    values: Vec<Vec<Symbol>>,
}

impl EngineSchema {
    fn parse(input: &str) -> Result<EngineSchema, GenericError> {
        if input.is_empty() || !input.is_ascii() {
            return Err(GenericError::new(
                "the input must consist of ascii character only",
            ));
        }

        let mut result: Vec<Vec<Symbol>> = Vec::new();
        let mut dimension_x: Option<usize> = None;
        for line in input.lines() {
            let bytes = line.as_bytes();
            if dimension_x.is_none() {
                dimension_x = Some(bytes.len());
            }

            if dimension_x.unwrap_or(0) != bytes.len() {
                return Err(GenericError::new(
                    "the input lines do not have the same length",
                ));
            }

            result.push(Symbol::parse_all(bytes));
        }

        return Ok(EngineSchema {
            dimension_x: dimension_x.unwrap_or(0),
            dimension_y: result.len(),
            values: result,
        });
    }

    fn number_sequences_in_row(&self, row: usize) -> Result<Vec<Range<usize>>, GenericError> {
        if row >= self.dimension_y {
            return Err(GenericError::new("row index out of range"));
        }

        let mut result: Vec<Range<usize>> = Vec::new();
        let mut current_range_start: Option<usize> = None;
        for (i, symbol) in self.values[row].iter().enumerate() {
            match symbol {
                Symbol::Number(_) => {
                    if current_range_start.is_some() {
                        // our current number sequence continues
                        continue;
                    }

                    // we are starting a new number sequence
                    current_range_start = Some(i);
                }
                _ => {
                    if current_range_start.is_none() {
                        // we are not in a number sequence, so we don't need to do anything
                        continue;
                    }

                    result.push(current_range_start.unwrap()..i);
                    current_range_start = None; // reset current number sequence
                }
            }
        }

        if current_range_start.is_some() {
            // we need to finish the last number sequence
            result.push(current_range_start.unwrap()..self.dimension_x);
        }

        return Ok(result);
    }

    fn number_sequence_counts(
        &self,
        row: usize,
        sequence_range: &Range<usize>,
    ) -> Result<bool, GenericError> {
        self.check_bounds(row, sequence_range)?;

        // check row above
        if row > 0 {
            let start = if sequence_range.start > 0 {
                sequence_range.start - 1
            } else {
                sequence_range.start
            };
            let end = if sequence_range.end < self.dimension_x {
                sequence_range.end + 1
            } else {
                sequence_range.end
            };

            for symbol in &self.values[row - 1][start..end] {
                if *symbol == Symbol::Other {
                    return Ok(true);
                }
            }
        }

        // check row below
        if row + 1 < self.dimension_y {
            let start = if sequence_range.start > 0 {
                sequence_range.start - 1
            } else {
                sequence_range.start
            };
            let end = if sequence_range.end < self.dimension_x {
                sequence_range.end + 1
            } else {
                sequence_range.end
            };

            for symbol in &self.values[row + 1][start..end] {
                if *symbol == Symbol::Other {
                    return Ok(true);
                }
            }
        }

        // check left from start
        if sequence_range.start > 0 {
            if self.values[row][sequence_range.start - 1] == Symbol::Other {
                return Ok(true);
            }
        }

        // check right from end
        if sequence_range.end < self.dimension_x {
            if self.values[row][sequence_range.end] == Symbol::Other {
                return Ok(true);
            }
        }

        return Ok(false);
    }

    fn number_sequence_to_number(
        &self,
        row: usize,
        sequence_range: &Range<usize>,
    ) -> Result<u32, GenericError> {
        self.check_bounds(row, sequence_range)?;

        let mut digits: Vec<u8> = Vec::new();
        for symbol in &self.values[row][sequence_range.start..sequence_range.end] {
            match *symbol {
                Symbol::Number(digit) => {
                    digits.push(digit);
                }
                _ => {
                    return Err(GenericError::new(
                        "the sequence range cannot be converted into a number",
                    ));
                }
            }
        }

        let mut result: u32 = 0;
        let base: u32 = 10;
        let mut exponent: u32 = (digits.len() - 1) as u32;
        for digit in digits {
            result += base.pow(exponent) * digit as u32;

            if exponent > 0 {
                exponent -= 1;
            }
        }

        return Ok(result);
    }

    fn check_bounds(&self, row: usize, sequence_range: &Range<usize>) -> Result<(), GenericError> {
        if row >= self.dimension_y {
            return Err(GenericError::new("row index out of range"));
        }

        if sequence_range.start >= self.dimension_x || sequence_range.end > self.dimension_x {
            return Err(GenericError::new("number sequence range invalid"));
        }

        return Ok(());
    }
}

pub fn day3_challenge1(file_path: &str) -> Result<u32, Box<dyn Error>> {
    let text = fs::read_to_string(file_path)?;
    let schema = EngineSchema::parse(&text)?;

    let mut result: u32 = 0;
    for row in 0..schema.dimension_y {
        let number_sequences = schema.number_sequences_in_row(row)?;
        for number_sequence in &number_sequences {
            if schema.number_sequence_counts(row, number_sequence)? {
                result += schema.number_sequence_to_number(row, number_sequence)?;
            }
        }
    }

    return Ok(result);
}

#[cfg(test)]
mod tests {
    use crate::day3::{EngineSchema, Symbol, DOT};

    #[test]
    fn parse_engine_schema_as_expected() {
        let input = "\
1.2.
..*.
3...";
        let actual = EngineSchema::parse(input).unwrap();

        assert_eq!(actual.dimension_x, 4);
        assert_eq!(actual.dimension_y, 3);
        assert_eq!(
            actual.values[0],
            vec![
                Symbol::Number(1),
                Symbol::Dot,
                Symbol::Number(2),
                Symbol::Dot
            ]
        );
        assert_eq!(
            actual.values[1],
            vec![Symbol::Dot, Symbol::Dot, Symbol::Other, Symbol::Dot]
        );
        assert_eq!(
            actual.values[2],
            vec![Symbol::Number(3), Symbol::Dot, Symbol::Dot, Symbol::Dot]
        );
    }

    #[test]
    fn parse_symbol() {
        assert_eq!(Symbol::parse(DOT), Symbol::Dot);
        assert_eq!(Symbol::parse(42), Symbol::Other); // that's a '*'
        assert_eq!(Symbol::parse(48), Symbol::Number(0));
        assert_eq!(Symbol::parse(49), Symbol::Number(1));
        assert_eq!(Symbol::parse(50), Symbol::Number(2));
        assert_eq!(Symbol::parse(51), Symbol::Number(3));
        assert_eq!(Symbol::parse(52), Symbol::Number(4));
        assert_eq!(Symbol::parse(53), Symbol::Number(5));
        assert_eq!(Symbol::parse(54), Symbol::Number(6));
        assert_eq!(Symbol::parse(55), Symbol::Number(7));
        assert_eq!(Symbol::parse(56), Symbol::Number(8));
        assert_eq!(Symbol::parse(57), Symbol::Number(9));
    }

    #[test]
    fn number_sequences_in_row() {
        let schema = EngineSchema {
            dimension_x: 5,
            dimension_y: 1,
            values: vec![vec![
                Symbol::Number(1),
                Symbol::Number(2),
                Symbol::Dot,
                Symbol::Other,
                Symbol::Number(3),
            ]],
        };
        let actual = schema.number_sequences_in_row(0).unwrap();

        assert_eq!(actual, vec![0..2, 4..5]);
    }

    #[test]
    fn number_sequence_counts_when_symbol_is_top_left() {
        let schema = EngineSchema {
            dimension_x: 2,
            dimension_y: 2,
            values: vec![
                vec![Symbol::Other, Symbol::Dot],
                vec![Symbol::Dot, Symbol::Number(1)],
            ],
        };
        let actual = schema.number_sequence_counts(1, &(1..2)).unwrap();

        assert!(actual);
    }

    #[test]
    fn number_sequence_counts_when_symbol_is_top_middle() {
        let schema = EngineSchema {
            dimension_x: 3,
            dimension_y: 3,
            values: vec![
                vec![Symbol::Dot, Symbol::Other, Symbol::Dot],
                vec![Symbol::Dot, Symbol::Number(1), Symbol::Dot],
            ],
        };
        let actual = schema.number_sequence_counts(1, &(1..2)).unwrap();

        assert!(actual);
    }

    #[test]
    fn number_sequence_counts_when_symbol_is_top_right() {
        let schema = EngineSchema {
            dimension_x: 3,
            dimension_y: 3,
            values: vec![
                vec![Symbol::Dot, Symbol::Dot, Symbol::Other],
                vec![Symbol::Dot, Symbol::Number(1), Symbol::Dot],
            ],
        };
        let actual = schema.number_sequence_counts(1, &(1..2)).unwrap();

        assert!(actual);
    }

    #[test]
    fn number_sequence_counts_when_symbol_is_left() {
        let schema = EngineSchema {
            dimension_x: 2,
            dimension_y: 1,
            values: vec![vec![Symbol::Other, Symbol::Number(1)]],
        };
        let actual = schema.number_sequence_counts(0, &(1..2)).unwrap();

        assert!(actual);
    }

    #[test]
    fn number_sequence_counts_when_symbol_is_right() {
        let schema = EngineSchema {
            dimension_x: 2,
            dimension_y: 1,
            values: vec![vec![Symbol::Number(1), Symbol::Other]],
        };
        let actual = schema.number_sequence_counts(0, &(0..1)).unwrap();

        assert!(actual);
    }

    #[test]
    fn number_sequence_counts_when_symbol_is_bottom_left() {
        let schema = EngineSchema {
            dimension_x: 2,
            dimension_y: 2,
            values: vec![
                vec![Symbol::Dot, Symbol::Number(1)],
                vec![Symbol::Other, Symbol::Dot],
            ],
        };
        let actual = schema.number_sequence_counts(0, &(1..2)).unwrap();

        assert!(actual);
    }

    #[test]
    fn number_sequence_counts_when_symbol_is_bottom_middle() {
        let schema = EngineSchema {
            dimension_x: 3,
            dimension_y: 2,
            values: vec![
                vec![Symbol::Dot, Symbol::Number(1), Symbol::Dot],
                vec![Symbol::Dot, Symbol::Other, Symbol::Dot],
            ],
        };
        let actual = schema.number_sequence_counts(0, &(1..2)).unwrap();

        assert!(actual);
    }

    #[test]
    fn number_sequence_counts_when_symbol_is_bottom_right() {
        let schema = EngineSchema {
            dimension_x: 3,
            dimension_y: 2,
            values: vec![
                vec![Symbol::Dot, Symbol::Number(1), Symbol::Dot],
                vec![Symbol::Dot, Symbol::Dot, Symbol::Other],
            ],
        };
        let actual = schema.number_sequence_counts(0, &(1..2)).unwrap();

        assert!(actual);
    }

    #[test]
    fn sequence_range_to_number() {
        let schema = EngineSchema {
            dimension_x: 3,
            dimension_y: 1,
            values: vec![vec![
                Symbol::Number(1),
                Symbol::Number(2),
                Symbol::Number(3),
            ]],
        };
        let actual = schema.number_sequence_to_number(0, &(0..3)).unwrap();

        assert_eq!(actual, 123);
    }
}
