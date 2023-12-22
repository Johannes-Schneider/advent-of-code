use bitvec::bitvec;
use bitvec::order::Msb0;
use bitvec::vec::BitVec;

use crate::day10::direction::Direction;
use crate::day10::direction::Direction::{East, North, South, West};
use crate::day10::tile::PipeShape;
use crate::day10::tile::PipeShape::{
    NorthToEast, NorthToSouth, NorthToWest, SouthToEast, SouthToWest, WestToEast,
};
use crate::day10::tile_map::TileMap;

pub struct LoopMap<'a> {
    pub tile_map: &'a TileMap,
    pub start_tile_index: usize,
    pub start_tile_shape: PipeShape,
    is_in_loop: BitVec<u8, Msb0>,
    pub length: usize,
}

impl LoopMap<'_> {
    pub fn find_first<'a>(
        tile_map: &'a TileMap,
        start_tile_index: &'a usize,
    ) -> Option<LoopMap<'a>> {
        let mut is_in_loop = bitvec![u8, Msb0;];
        for _ in 0..tile_map.tiles.len() {
            is_in_loop.push(false);
        }

        let maybe_path = LoopMap::find_first_recursive(tile_map, vec![*start_tile_index]);
        if maybe_path.is_none() {
            return None;
        }

        let path = maybe_path.unwrap();
        let start_tile_shape = LoopMap::get_start_tile_shape(&path)?;
        let length = path.len();
        for index in path {
            let mut bit = is_in_loop.get_mut(index).unwrap();
            *bit = true;
        }

        return Some(LoopMap {
            tile_map,
            start_tile_index: *start_tile_index,
            start_tile_shape,
            is_in_loop,
            length,
        });
    }

    fn find_first_recursive(tile_map: &TileMap, current_path: Vec<usize>) -> Option<Vec<usize>> {
        let current_path_len = current_path.len();
        let current_index = &current_path[current_path_len - 1];
        let previous_index = if current_path_len > 1 {
            Some(&current_path[current_path_len - 2])
        } else {
            None
        };
        let neighbors = LoopMap::get_neighbors_excluding(tile_map, current_index, previous_index);

        for neighbor in neighbors {
            if &neighbor == &current_path[0] {
                // our next neighbor is the start node, so the loop is complete
                return Some(current_path);
            }

            let mut next_path = current_path.to_vec();
            next_path.push(neighbor);

            let result = LoopMap::find_first_recursive(tile_map, next_path);
            if result.is_some() {
                return result;
            }
        }

        return None;
    }

    fn get_neighbors_excluding(
        tile_map: &TileMap,
        current_node: &usize,
        previous_node: Option<&usize>,
    ) -> Vec<usize> {
        return (&tile_map.neighbor_indices[*current_node])
            .iter()
            .filter(|i| {
                *i != current_node && (previous_node.is_none() || *i != previous_node.unwrap())
            })
            .map(|i| *i)
            .collect();
    }

    fn get_start_tile_shape(path: &Vec<usize>) -> Option<PipeShape> {
        let start_index = &path[0];
        let first_neighbor_index = &path[1];
        let second_neighbor_index = &path[path.len() - 1];

        let first_diff = (*first_neighbor_index as i128) - (*start_index as i128);
        let first_direction = LoopMap::get_relative_direction(first_diff);

        let second_diff = (*second_neighbor_index as i128) - (*start_index as i128);
        let second_direction = LoopMap::get_relative_direction(second_diff);

        if first_direction == North && second_direction == South
            || first_direction == South && second_direction == North
        {
            return Some(NorthToSouth);
        }

        if first_direction == North && second_direction == East
            || first_direction == East && second_direction == North
        {
            return Some(NorthToEast);
        }

        if first_direction == North && second_direction == West
            || first_direction == West && second_direction == North
        {
            return Some(NorthToWest);
        }

        if first_direction == South && second_direction == East
            || first_direction == East && second_direction == South
        {
            return Some(SouthToEast);
        }

        if first_direction == South && second_direction == West
            || first_direction == West && second_direction == South
        {
            return Some(SouthToWest);
        }

        if first_direction == West && second_direction == East
            || first_direction == East && second_direction == West
        {
            return Some(WestToEast);
        }

        return None;
    }

    fn get_relative_direction(index_difference: i128) -> Direction {
        if index_difference == -1 {
            // one to the left
            return West;
        }

        if index_difference == 1 {
            // one to the right
            return East;
        }

        if index_difference < 0 {
            // one row up
            return North;
        }

        return South;
    }

    pub fn contains_2d(&self, row_index: usize, column_index: usize) -> bool {
        let tile_index = self.tile_map.number_of_columns * row_index + column_index;
        return self.contains_1d(tile_index);
    }

    pub fn contains_1d(&self, tile_index: usize) -> bool {
        return self.is_in_loop[tile_index];
    }
}

#[cfg(test)]
mod tests {
    use crate::day10::loop_map::LoopMap;
    use crate::day10::tile::PipeShape;
    use crate::day10::tile_map::TileMap;

    #[test]
    fn test_find_first_001() {
        let input = "\
.....
.S-7.
.|.|.
.L-J.
.....";

        let tile_map = TileMap::parse(&input).unwrap();
        let sut = LoopMap::find_first(&tile_map, &6).unwrap();

        assert_eq!(sut.length, 8);
        assert_eq!(sut.start_tile_shape, PipeShape::SouthToEast);
    }
}
