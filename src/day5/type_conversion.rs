use std::cmp::Ordering;
use std::cmp::Ordering::Equal;

use crate::day5::r#type::Type;
use crate::string_functions::{split_and_clean, to_u128};
use crate::GenericError;

pub struct TypeConversion {
    source: Type,
    destination: Type,
    mappings: Vec<RangeMapping>,
}

#[derive(Debug, PartialEq, Clone)]
struct RangeMapping {
    source_offset: u128,
    destination_offset: u128,
    length: Option<u128>, // NONE means that the range has no end
}

impl TypeConversion {
    pub fn parse(input_lines: &[&str]) -> Result<TypeConversion, GenericError> {
        if input_lines.len() < 2 {
            return Err(GenericError::new("cannot extract type conversion"));
        }

        let mapping_parts = split_and_clean(input_lines[0], " ");
        if mapping_parts.len() != 2 {
            return Err(GenericError::new(
                "cannot extract source and destination type",
            ));
        }

        let sub_parts = split_and_clean(mapping_parts[0], "-");
        if sub_parts.len() != 3 {
            return Err(GenericError::new(
                "cannot extract source and destination type",
            ));
        }

        let source_type = Type::from_string(sub_parts[0])?;
        let destination_type = Type::from_string(sub_parts[2])?;
        let range_mappings = TypeConversion::parse_mappings(&input_lines[1..])?;

        return Ok(TypeConversion {
            source: source_type,
            destination: destination_type,
            mappings: range_mappings,
        });
    }

    fn parse_mappings(input: &[&str]) -> Result<Vec<RangeMapping>, GenericError> {
        let mut range_mappings: Vec<RangeMapping> = Vec::new();

        for line in input {
            range_mappings.push(RangeMapping::parse(line)?);
        }

        range_mappings.sort_by(|r1, r2| r1.source_offset.cmp(&r2.source_offset));
        TypeConversion::add_missing_range_mappings(&mut range_mappings)?;

        return Ok(range_mappings);
    }

    fn add_missing_range_mappings(mappings: &mut Vec<RangeMapping>) -> Result<(), GenericError> {
        let mut next_source_index: u128 = 0;
        let mut index: usize = 0;
        while index < mappings.len() {
            let mapping = mappings[index].clone();
            if mapping.length.is_none() {
                return Err(GenericError::new("found unexpected unbound range mapping"));
            }

            if mapping.source_offset > next_source_index {
                let length = Some(mapping.source_offset - next_source_index);

                // we are missing a mapping
                mappings.insert(
                    index,
                    RangeMapping {
                        source_offset: next_source_index,
                        destination_offset: next_source_index,
                        length,
                    },
                );
                index += 1; // skip the next index (which is what we were just looking at)
            }

            next_source_index = mapping.source_offset + mapping.length.unwrap();
            index += 1;
        }

        mappings.push(RangeMapping {
            source_offset: next_source_index,
            destination_offset: next_source_index,
            length: None,
        });

        return Ok(());
    }

    pub fn convert(&self, source_value: u128) -> Result<u128, GenericError> {
        let mapping = self.find_mapping(source_value)?;
        let source_distance = source_value as i128 - mapping.source_offset as i128;
        return Ok((mapping.destination_offset as i128 + source_distance) as u128);
    }

    fn find_mapping(&self, source_value: u128) -> Result<&RangeMapping, GenericError> {
        if self.mappings.len() < 128 {
            return self.find_mapping_linear(source_value);
        }

        return self.find_mapping_binary(source_value);
    }

    fn find_mapping_linear(&self, source_value: u128) -> Result<&RangeMapping, GenericError> {
        for mapping in &self.mappings {
            if mapping.find(source_value) == Equal {
                return Ok(mapping);
            }
        }

        return Err(GenericError::new("unable to find range mapping"));
    }

    fn find_mapping_binary(&self, source_value: u128) -> Result<&RangeMapping, GenericError> {
        let mut lower = 0;
        let mut upper = self.mappings.len();

        while upper >= lower {
            let index = lower + ((upper - lower) / 2);
            let result = self.mappings[index].find(source_value);

            match result {
                Equal => return Ok(&self.mappings[index]),
                Ordering::Less => upper = index,
                Ordering::Greater => lower = index,
            };
        }

        return Err(GenericError::new("unable to find range mapping"));
    }
}

impl RangeMapping {
    fn parse(input: &str) -> Result<RangeMapping, GenericError> {
        let parts = split_and_clean(input, " ");
        if parts.len() != 3 {
            return Err(GenericError::new("unexpected range mapping input"));
        }

        let destination_offset = to_u128(parts[0])?;
        let source_offset = to_u128(parts[1])?;
        let length = to_u128(parts[2])?;

        return Ok(RangeMapping {
            source_offset,
            destination_offset,
            length: Some(length),
        });
    }

    fn find(&self, source_value: u128) -> Ordering {
        if source_value < self.source_offset {
            return Ordering::Less;
        }

        if self.length.is_none() {
            return Equal;
        }

        if source_value < self.source_offset + self.length.unwrap() {
            return Equal;
        }

        return Ordering::Greater;
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;

    use crate::day5::r#type::Type;
    use crate::day5::type_conversion::{RangeMapping, TypeConversion};

    #[test]
    fn test_parse() {
        let input = "\
soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15"
            .lines()
            .collect::<Vec<&str>>();
        let sut = TypeConversion::parse(&input).unwrap();

        assert_eq!(sut.source, Type::SOIL);
        assert_eq!(sut.destination, Type::FERTILIZER);
        assert_eq!(
            sut.mappings,
            vec![
                RangeMapping {
                    source_offset: 0,
                    destination_offset: 39,
                    length: Some(15),
                },
                RangeMapping {
                    source_offset: 15,
                    destination_offset: 0,
                    length: Some(37),
                },
                RangeMapping {
                    source_offset: 52,
                    destination_offset: 37,
                    length: Some(2),
                },
                RangeMapping {
                    source_offset: 54,
                    destination_offset: 54,
                    length: None,
                },
            ]
        );
    }

    #[test]
    fn test_add_missing_range_mappings() {
        let mut sut = vec![
            RangeMapping {
                source_offset: 10,
                destination_offset: 20,
                length: Some(10),
            },
            RangeMapping {
                source_offset: 30,
                destination_offset: 0,
                length: Some(5),
            },
            RangeMapping {
                source_offset: 90,
                destination_offset: 45,
                length: Some(2),
            },
        ];
        TypeConversion::add_missing_range_mappings(&mut sut).unwrap();

        assert_eq!(
            sut,
            vec![
                RangeMapping {
                    source_offset: 0,
                    destination_offset: 0,
                    length: Some(10),
                },
                RangeMapping {
                    source_offset: 10,
                    destination_offset: 20,
                    length: Some(10),
                },
                RangeMapping {
                    source_offset: 20,
                    destination_offset: 20,
                    length: Some(10),
                },
                RangeMapping {
                    source_offset: 30,
                    destination_offset: 0,
                    length: Some(5),
                },
                RangeMapping {
                    source_offset: 35,
                    destination_offset: 35,
                    length: Some(55),
                },
                RangeMapping {
                    source_offset: 90,
                    destination_offset: 45,
                    length: Some(2),
                },
                RangeMapping {
                    source_offset: 92,
                    destination_offset: 92,
                    length: None,
                },
            ]
        );
    }

    #[test]
    fn test_convert() {
        let input = "\
soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15"
            .lines()
            .collect::<Vec<&str>>();
        let sut = TypeConversion::parse(&input).unwrap();

        for value in 0..15 {
            assert_eq!(
                sut.convert(value).unwrap(),
                ((39i128 - 0i128) + value as i128) as u128
            );
        }

        for value in 15..52 {
            assert_eq!(
                sut.convert(value).unwrap(),
                ((0i128 - 15i128) + value as i128) as u128
            );
        }

        for value in 52..54 {
            assert_eq!(
                sut.convert(value).unwrap(),
                ((37i128 - 52i128) + value as i128) as u128
            );
        }

        for value in 54..100 {
            assert_eq!(sut.convert(value).unwrap(), value);
        }
    }

    #[test]
    fn test_find_mapping_linear() {
        let input = "\
soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15"
            .lines()
            .collect::<Vec<&str>>();
        let sut = TypeConversion::parse(&input).unwrap();

        for source_value in 0..15 {
            assert_eq!(
                sut.find_mapping_linear(source_value).unwrap(),
                &sut.mappings[0]
            );
        }

        for source_value in 15..52 {
            assert_eq!(
                sut.find_mapping_linear(source_value).unwrap(),
                &sut.mappings[1]
            );
        }

        for source_value in 52..54 {
            assert_eq!(
                sut.find_mapping_linear(source_value).unwrap(),
                &sut.mappings[2]
            );
        }

        for source_value in 54..100 {
            assert_eq!(
                sut.find_mapping_linear(source_value).unwrap(),
                &sut.mappings[3]
            );
        }
    }

    #[test]
    fn test_find_mapping_binary() {
        let input = "\
soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15"
            .lines()
            .collect::<Vec<&str>>();
        let sut = TypeConversion::parse(&input).unwrap();

        for source_value in 0..15 {
            assert_eq!(
                sut.find_mapping_binary(source_value).unwrap(),
                &sut.mappings[0]
            );
        }

        for source_value in 15..52 {
            assert_eq!(
                sut.find_mapping_binary(source_value).unwrap(),
                &sut.mappings[1]
            );
        }

        for source_value in 52..54 {
            assert_eq!(
                sut.find_mapping_binary(source_value).unwrap(),
                &sut.mappings[2]
            );
        }

        for source_value in 54..100 {
            assert_eq!(
                sut.find_mapping_binary(source_value).unwrap(),
                &sut.mappings[3]
            );
        }
    }

    #[test]
    fn test_parse_range_mapping() {
        let input = "50 98 2";
        let actual = RangeMapping::parse(input).unwrap();

        assert_eq!(
            actual,
            RangeMapping {
                destination_offset: 50,
                source_offset: 98,
                length: Some(2),
            }
        );
    }

    #[test]
    fn test_range_mapping_with_limited_length_find() {
        let sut = RangeMapping {
            source_offset: 10,
            destination_offset: 39,
            length: Some(15),
        };

        for source_value in 0..10 {
            assert_eq!(sut.find(source_value), Ordering::Less);
        }

        for source_value in 10..25 {
            assert_eq!(sut.find(source_value), Ordering::Equal);
        }

        for source_value in 25..100 {
            assert_eq!(sut.find(source_value), Ordering::Greater);
        }
    }

    #[test]
    fn test_range_mapping_with_unlimited_length_find() {
        let sut = RangeMapping {
            source_offset: 10,
            destination_offset: 39,
            length: None,
        };

        for source_value in 0..10 {
            assert_eq!(sut.find(source_value), Ordering::Less);
        }

        for source_value in 10..100 {
            assert_eq!(sut.find(source_value), Ordering::Equal);
        }
    }
}
