use std::fmt::{Display, Formatter};

use crate::day12::spring::Spring;
use crate::string_functions::{all_to_usize, split_and_clean};
use crate::GenericError;

pub trait RecordRowView {
    fn number_of_springs(&self) -> usize;
    fn get_spring(&self, index: usize) -> &Spring;
    fn number_of_groups(&self) -> usize;
    fn get_group(&self, index: usize) -> usize;
}

pub struct UnfoldedRecordRowView<'a> {
    row: &'a RecordRow,
    fold_factor: usize,
}

impl RecordRowView for UnfoldedRecordRowView<'_> {
    fn number_of_springs(&self) -> usize {
        return self.row.springs.len() * self.fold_factor + self.fold_factor - 1;
        // add `fold_factor - 1` unknown springs
    }

    fn get_spring(&self, index: usize) -> &Spring {
        let relative_index = index % (self.row.springs.len() + 1);
        if relative_index == self.row.springs.len() {
            return &Spring::Unknown;
        }

        return &self.row.springs[relative_index];
    }

    fn number_of_groups(&self) -> usize {
        return self.row.broken_spring_groups.len() * self.fold_factor;
    }

    fn get_group(&self, index: usize) -> usize {
        let relative_index = index % self.row.broken_spring_groups.len();
        return *&self.row.broken_spring_groups[relative_index];
    }
}

impl UnfoldedRecordRowView<'_> {
    pub fn new(row: &RecordRow, fold_factor: usize) -> UnfoldedRecordRowView {
        return UnfoldedRecordRowView { row, fold_factor };
    }
}

pub struct RecordRow {
    springs: Vec<Spring>,
    broken_spring_groups: Vec<usize>,
}

impl RecordRow {
    pub fn parse_all(input: &str) -> Result<Vec<RecordRow>, GenericError> {
        let mut result = Vec::new();
        for line in input.lines() {
            result.push(RecordRow::parse(line)?);
        }

        return Ok(result);
    }

    pub fn parse(input: &str) -> Result<RecordRow, GenericError> {
        let parts = split_and_clean(input, " ");
        if parts.len() != 2 {
            return Err(GenericError::new("incorrect record row format"));
        }

        let springs = RecordRow::parse_springs(&parts[0])?;

        let raw_spring_groups = split_and_clean(&parts[1], ",");
        let broken_spring_groups = all_to_usize(&raw_spring_groups)?;

        return Ok(RecordRow {
            springs,
            broken_spring_groups,
        });
    }

    fn parse_springs(input: &str) -> Result<Vec<Spring>, GenericError> {
        let mut result = Vec::new();
        for byte in input.as_bytes() {
            result.push(Spring::parse(byte)?);
        }

        return Ok(result);
    }
}

impl Display for RecordRow {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut _result = write!(f, "");

        for spring in &self.springs {
            _result = write!(f, "{spring}");
        }

        _result = write!(f, " ");

        let groups = self
            .broken_spring_groups
            .iter()
            .map(|i| i.to_string())
            .collect::<Vec<String>>()
            .join(",");
        return write!(f, "{groups}");
    }
}

#[cfg(test)]
mod tests {
    use crate::day12::record_row::{RecordRow, RecordRowView, UnfoldedRecordRowView};
    use crate::day12::spring::Spring;

    #[test]
    fn test_number_of_unfolded_springs_001() {
        let input = ".# 1";
        let row = RecordRow::parse(&input).unwrap();

        let sut = UnfoldedRecordRowView::new(&row, 1);

        assert_eq!(sut.number_of_springs(), 2);
        assert_eq!(sut.number_of_groups(), 1);

        assert_eq!(sut.get_spring(0), &Spring::Functioning);
        assert_eq!(sut.get_spring(1), &Spring::Broken);
        assert_eq!(sut.get_group(0), 1);
    }

    #[test]
    fn test_number_of_unfolded_springs_002() {
        let input = ".# 1";
        let row = RecordRow::parse(&input).unwrap();

        let sut = UnfoldedRecordRowView::new(&row, 5);

        assert_eq!(sut.number_of_springs(), 14);
        assert_eq!(sut.number_of_groups(), 5);

        assert_eq!(sut.get_spring(0), &Spring::Functioning);
        assert_eq!(sut.get_spring(1), &Spring::Broken);
        assert_eq!(sut.get_spring(2), &Spring::Unknown);
        assert_eq!(sut.get_spring(3), &Spring::Functioning);
        assert_eq!(sut.get_spring(4), &Spring::Broken);
        assert_eq!(sut.get_spring(5), &Spring::Unknown);
        assert_eq!(sut.get_spring(13), &Spring::Broken);

        assert_eq!(sut.get_group(0), 1);
        assert_eq!(sut.get_group(1), 1);
        assert_eq!(sut.get_group(2), 1);
    }
}
