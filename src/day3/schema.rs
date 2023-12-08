use crate::day3::row_slice::RowSlice;
use advent_of_code::GenericError;

use crate::day3::symbol::Symbol;

#[derive(Debug, PartialEq)]
pub struct Schema {
    pub rows: usize,
    pub columns: usize,
    pub symbols: Vec<Vec<Symbol>>,
}

impl Schema {
    pub fn parse(input: &str) -> Result<Schema, GenericError> {
        if !input.is_ascii() {
            return Err(GenericError::new(
                "the input must consist of ascii chars only",
            ));
        }

        let mut symbols: Vec<Vec<Symbol>> = Vec::new();
        let mut columns: Option<usize> = None;
        for line in input.lines() {
            let bytes = line.as_bytes();
            if columns.is_none() {
                columns = Some(bytes.len());
            }

            if columns.unwrap_or(0) != bytes.len() {
                return Err(GenericError::new("all lines must be equally long"));
            }

            symbols.push(Symbol::parse_all(bytes));
        }

        Ok(Schema {
            rows: symbols.len(),
            columns: columns.unwrap_or(0),
            symbols,
        })
    }

    pub fn extract_number_slices(&self) -> Vec<RowSlice> {
        let mut result: Vec<RowSlice> = Vec::new();
        for row in 0..self.rows {
            let mut slices = self.extract_number_slices_from_row(row);
            result.append(&mut slices);
        }

        return result;
    }

    fn extract_number_slices_from_row(&self, row: usize) -> Vec<RowSlice> {
        let mut result: Vec<RowSlice> = Vec::new();

        let mut sequence_start: Option<usize> = None;
        for (i, symbol) in self.symbols[row].iter().enumerate() {
            match *symbol {
                Symbol::Number(_) => {
                    if sequence_start.is_none() {
                        // start a new sequence
                        sequence_start = Some(i);
                    }
                }
                _ => {
                    if sequence_start.is_some() {
                        // end current sequence
                        result.push(RowSlice::new(self, row, sequence_start.unwrap(), i));
                        sequence_start = None;
                    }
                }
            }
        }

        if sequence_start.is_some() {
            // end last sequence
            result.push(RowSlice::new(
                self,
                row,
                sequence_start.unwrap(),
                self.columns,
            ));
        }

        return result;
    }
}

#[cfg(test)]
mod tests {
    use crate::day3::row_slice::RowSlice;
    use crate::day3::schema::Schema;
    use crate::day3::symbol::Symbol;

    #[test]
    fn test_parse() {
        let input = "\
1.2.
..*.
3.#.";
        let actual = Schema::parse(input).unwrap();

        assert_eq!(actual.rows, 3);
        assert_eq!(actual.columns, 4);
        assert_eq!(
            actual.symbols[0],
            vec![
                Symbol::Number(1),
                Symbol::Dot,
                Symbol::Number(2),
                Symbol::Dot,
            ]
        );
        assert_eq!(
            actual.symbols[1],
            vec![Symbol::Dot, Symbol::Dot, Symbol::Other(42), Symbol::Dot]
        );
        assert_eq!(
            actual.symbols[2],
            vec![
                Symbol::Number(3),
                Symbol::Dot,
                Symbol::Other(35),
                Symbol::Dot
            ]
        );
    }

    #[test]
    fn test_extract_number_slices() {
        let schema = Schema::parse(
            "\
..1.
12..
3..4",
        )
        .unwrap();
        let actual = schema.extract_number_slices();

        assert_eq!(
            actual,
            vec![
                RowSlice::new(&schema, 0, 2, 3),
                RowSlice::new(&schema, 1, 0, 2),
                RowSlice::new(&schema, 2, 0, 1),
                RowSlice::new(&schema, 2, 3, 4)
            ]
        );
    }
}
