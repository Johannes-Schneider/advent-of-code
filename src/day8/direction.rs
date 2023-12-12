use crate::GenericError;
use phf::phf_map;

static DIRECTIONS: phf::Map<u8, Direction> = phf_map! {
    b'L' => Direction::Left,
    b'R' => Direction::Right
};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Direction {
    Left,
    Right,
}

impl Direction {
    pub fn parse_all(input: &str) -> Result<Vec<Direction>, GenericError> {
        let mut result: Vec<Direction> = Vec::new();
        for byte in input.as_bytes() {
            let direction = DIRECTIONS.get(byte);
            if direction.is_none() {
                return Err(GenericError::new("unable to parse direction"));
            }

            result.push(*direction.unwrap());
        }

        return Ok(result);
    }
}

#[cfg(test)]
mod tests {
    use crate::day8::direction::Direction;

    #[test]
    fn test_parse_all() {
        let input = "LLRRLL";
        let actual = Direction::parse_all(&input).unwrap();

        assert_eq!(
            actual,
            vec![
                Direction::Left,
                Direction::Left,
                Direction::Right,
                Direction::Right,
                Direction::Left,
                Direction::Left
            ]
        );
    }
}
