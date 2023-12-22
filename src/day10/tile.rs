use std::collections::HashMap;

use lazy_static::lazy_static;

use crate::day10::direction::Direction;
use crate::GenericError;

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub enum PipeShape {
    NorthToSouth,
    NorthToEast,
    NorthToWest,
    SouthToEast,
    SouthToWest,
    WestToEast,
}

#[derive(Copy, Clone)]
pub enum Tile {
    Ground,
    Start,
    Pipe(PipeShape),
}

lazy_static! {
    static ref PIPE_DIRECTIONS: HashMap<PipeShape, Vec<Direction>> = {
        let mut map = HashMap::new();
        map.insert(
            PipeShape::NorthToSouth,
            vec![Direction::North, Direction::South],
        );
        map.insert(
            PipeShape::NorthToEast,
            vec![Direction::North, Direction::East],
        );
        map.insert(
            PipeShape::NorthToWest,
            vec![Direction::North, Direction::West],
        );
        map.insert(
            PipeShape::SouthToEast,
            vec![Direction::South, Direction::East],
        );
        map.insert(
            PipeShape::SouthToWest,
            vec![Direction::South, Direction::West],
        );
        map.insert(
            PipeShape::WestToEast,
            vec![Direction::West, Direction::East],
        );

        return map;
    };
    static ref ALL_DIRECTIONS: Vec<Direction> = vec![
        Direction::North,
        Direction::East,
        Direction::South,
        Direction::West
    ];
    static ref NO_DIRECTIONS: Vec<Direction> = Vec::new();
}

impl PipeShape {
    pub fn directions(&self) -> &Vec<Direction> {
        return PIPE_DIRECTIONS.get(self).unwrap();
    }
}

impl Tile {
    pub fn parse(input: &u8) -> Result<Tile, GenericError> {
        return match input {
            b'|' => Ok(Tile::Pipe(PipeShape::NorthToSouth)),
            b'-' => Ok(Tile::Pipe(PipeShape::WestToEast)),
            b'L' => Ok(Tile::Pipe(PipeShape::NorthToEast)),
            b'J' => Ok(Tile::Pipe(PipeShape::NorthToWest)),
            b'7' => Ok(Tile::Pipe(PipeShape::SouthToWest)),
            b'F' => Ok(Tile::Pipe(PipeShape::SouthToEast)),
            b'.' => Ok(Tile::Ground),
            b'S' => Ok(Tile::Start),
            _ => Err(GenericError::new("unknown tile type")),
        };
    }

    pub fn can_reach(&self, other: &Tile, relative_direction: &Direction) -> bool {
        let my_directions = self.directions();
        if !my_directions.contains(relative_direction) {
            // we are not pointing in the relative_direction
            return false;
        }

        return other.directions().contains(&relative_direction.inverse());
    }

    pub fn directions(&self) -> &Vec<Direction> {
        return match self {
            Tile::Ground => &NO_DIRECTIONS,
            Tile::Start => &ALL_DIRECTIONS,
            Tile::Pipe(shape) => shape.directions(),
        };
    }
}
