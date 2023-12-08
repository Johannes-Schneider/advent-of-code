use std::cmp::{max, min};

use advent_of_code::GenericError;

use crate::day3::schema::Schema;
use crate::day3::symbol::Symbol;

#[derive(Debug, PartialEq)]
pub struct RowSlice<'a> {
    schema: &'a Schema,
    row: usize,
    start: usize,
    end: usize, // excluded!
}

impl RowSlice<'_> {
    pub fn new(schema: &Schema, row: usize, start: usize, end: usize) -> RowSlice {
        RowSlice {
            schema,
            row,
            start,
            end,
        }
    }

    pub fn symbols(&self) -> &[Symbol] {
        &self.schema.symbols[self.row][self.start..self.end]
    }

    pub fn is_number(&self) -> bool {
        self.symbols().iter().all(|s| match s {
            Symbol::Number(_) => true,
            _ => false,
        })
    }

    pub fn to_number(&self) -> u32 {
        self.try_to_number()
            .expect("slice is expected to be a numer")
    }

    pub fn try_to_number(&self) -> Result<u32, GenericError> {
        let symbols = self.symbols();
        let mut exponent: u32 = (symbols.len() - 1) as u32;
        let base: u32 = 10;

        let mut result: u32 = 0;
        for symbol in symbols {
            match *symbol {
                Symbol::Number(value) => {
                    result += base.pow(exponent) * (value as u32);

                    if exponent > 0 {
                        exponent -= 1;
                    }
                }
                _ => return Err(GenericError::new("row slice is not a number")),
            }
        }

        return Ok(result);
    }

    pub fn is_part_number(&self) -> bool {
        if !self.is_number() {
            return false;
        }

        let adjacent_start = if self.start > 0 {
            self.start - 1
        } else {
            self.start
        };
        let adjacent_end = if self.end + 1 < self.schema.columns {
            self.end + 1
        } else {
            self.end
        };

        // check row above
        if self.row > 0 {
            if self.schema.symbols[self.row - 1][adjacent_start..adjacent_end]
                .iter()
                .any(|s| match s {
                    Symbol::Other(_) => true,
                    _ => false,
                })
            {
                return true;
            }
        }

        // check row below
        if self.row + 1 < self.schema.rows {
            if self.schema.symbols[self.row + 1][adjacent_start..adjacent_end]
                .iter()
                .any(|s| match s {
                    Symbol::Other(_) => true,
                    _ => false,
                })
            {
                return true;
            }
        }

        // check left
        if self.start > 0 {
            match self.schema.symbols[self.row][self.start - 1] {
                Symbol::Other(_) => return true,
                _ => {}
            }
        }

        // check right
        if self.end < self.schema.columns {
            match self.schema.symbols[self.row][self.end] {
                Symbol::Other(_) => return true,
                _ => {}
            }
        }

        return false;
    }

    pub fn is_adjacent_to_cell(&self, row: usize, column: usize) -> bool {
        let row_distance = self.row_distance(row);
        if row_distance > 1 {
            return false;
        }

        let adjacent_start = if self.start > 0 {
            self.start - 1
        } else {
            self.start
        };

        return column >= adjacent_start && column <= self.end;
    }

    fn row_distance(&self, row: usize) -> usize {
        max(self.row, row) - min(self.row, row)
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Range;

    use crate::day3::row_slice::RowSlice;
    use crate::day3::schema::Schema;
    use crate::day3::symbol::Symbol;

    #[test]
    fn test_symbols() {
        let schema = Schema::parse(".*#1").unwrap();
        let sut = RowSlice::new(&schema, 0, 0, 4);

        let actual = sut.symbols();
        assert_eq!(
            actual,
            vec![
                Symbol::Dot,
                Symbol::Other(42),
                Symbol::Other(35),
                Symbol::Number(1),
            ]
        );
    }

    #[test]
    fn test_is_number_true() {
        {
            // true
            let schema = Schema::parse("1234").unwrap();
            let sut = RowSlice::new(&schema, 0, 0, 4);

            assert_eq!(sut.is_number(), true);
        }

        {
            // false
            let schema = Schema::parse("12.4").unwrap();
            let sut = RowSlice::new(&schema, 0, 0, 4);

            assert_eq!(sut.is_number(), false);
        }
    }

    #[test]
    fn test_try_to_number() {
        {
            // should succeed
            let schema = Schema::parse("1234").unwrap();
            let sut = RowSlice::new(&schema, 0, 0, 4);

            assert_eq!(sut.try_to_number().unwrap(), 1234);
        }

        {
            // should fail
            let schema = Schema::parse("12.4").unwrap();
            let sut = RowSlice::new(&schema, 0, 0, 4);

            assert_eq!(sut.try_to_number().is_err(), true);
        }
    }

    #[test]
    fn test_is_adjacent_to_cell() {
        let schema = Schema::parse(
            "\
.....
.....
.....",
        )
        .unwrap();

        let slice = RowSlice {
            schema: &schema,
            row: 1,
            start: 2,
            end: 3,
        };

        // row 0
        assert_eq!(slice.is_adjacent_to_cell(0, 0), false);
        assert_eq!(slice.is_adjacent_to_cell(0, 1), true);
        assert_eq!(slice.is_adjacent_to_cell(0, 2), true);
        assert_eq!(slice.is_adjacent_to_cell(0, 3), true);
        assert_eq!(slice.is_adjacent_to_cell(0, 4), false);
        // row 1
        assert_eq!(slice.is_adjacent_to_cell(1, 0), false);
        assert_eq!(slice.is_adjacent_to_cell(1, 1), true);
        assert_eq!(slice.is_adjacent_to_cell(1, 2), true);
        assert_eq!(slice.is_adjacent_to_cell(1, 3), true);
        assert_eq!(slice.is_adjacent_to_cell(1, 4), false);
        // row 2
        assert_eq!(slice.is_adjacent_to_cell(2, 0), false);
        assert_eq!(slice.is_adjacent_to_cell(2, 1), true);
        assert_eq!(slice.is_adjacent_to_cell(2, 2), true);
        assert_eq!(slice.is_adjacent_to_cell(2, 3), true);
        assert_eq!(slice.is_adjacent_to_cell(2, 4), false);
    }

    fn assert_adjacent_cell(slice: &RowSlice, cell_row: usize, cell_column: usize, expected: bool) {
        assert_eq!(slice.is_adjacent_to_cell(cell_row, cell_column), expected);
    }

    #[test]
    fn test_is_part_number() {
        let schema = Schema::parse(
            "\
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..",
        )
        .unwrap();

        {
            // top left
            let slice = RowSlice {
                schema: &schema,
                row: 0,
                start: 0,
                end: 3,
            };
            assert_eq!(slice.is_part_number(), true);
        }

        {
            // top right
            let slice = RowSlice {
                schema: &schema,
                row: 0,
                start: 5,
                end: 8,
            };
            assert_eq!(slice.is_part_number(), false);
        }
    }
}
