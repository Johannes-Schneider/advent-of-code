use crate::day10::direction::Direction::{East, North, South, West};

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn connects_to(&self, next_direction: &Direction) -> bool {
        return next_direction == &self.inverse();
    }

    pub fn inverse(&self) -> Direction {
        match self {
            North => South,
            East => West,
            South => North,
            West => East,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::day10::direction::Direction::{East, North, South, West};

    #[test]
    fn test_connects_to() {
        assert_eq!(North.connects_to(&North), false);
        assert_eq!(North.connects_to(&East), false);
        assert_eq!(North.connects_to(&South), true);
        assert_eq!(North.connects_to(&West), false);

        assert_eq!(East.connects_to(&North), false);
        assert_eq!(East.connects_to(&East), false);
        assert_eq!(East.connects_to(&South), false);
        assert_eq!(East.connects_to(&West), true);

        assert_eq!(South.connects_to(&North), true);
        assert_eq!(South.connects_to(&East), false);
        assert_eq!(South.connects_to(&South), false);
        assert_eq!(South.connects_to(&West), false);

        assert_eq!(West.connects_to(&North), false);
        assert_eq!(West.connects_to(&East), true);
        assert_eq!(West.connects_to(&South), false);
        assert_eq!(West.connects_to(&West), false);
    }
}
