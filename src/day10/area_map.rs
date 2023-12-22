use bitvec::bitvec;
use bitvec::order::Msb0;

use crate::day10::loop_map::LoopMap;
use crate::day10::tile::PipeShape::{
    NorthToEast, NorthToSouth, NorthToWest, SouthToEast, SouthToWest,
};
use crate::day10::tile::Tile;
use crate::day10::tile::Tile::Pipe;

pub struct AreaMap {
    pub area_within: usize,
}

impl AreaMap {
    pub fn calculate(loop_map: &LoopMap) -> AreaMap {
        let mut is_within_loop = bitvec![u8, Msb0;];
        for _ in 0..loop_map.tile_map.tiles.len() {
            is_within_loop.push(false);
        }

        let mut area_within = 0usize;
        for row in 0..loop_map.tile_map.number_of_rows {
            let tiles = AreaMap::tiles_within_loop_in_row(loop_map, row);
            for tile_index in tiles {
                let mut bit = is_within_loop.get_mut(tile_index).unwrap();
                *bit = true;
                area_within += 1;
            }
        }

        return AreaMap { area_within };
    }

    fn tiles_within_loop_in_row(loop_map: &LoopMap, row: usize) -> Vec<usize> {
        let index_offset = loop_map.tile_map.number_of_columns * row;
        let mut tile_indices = Vec::new();
        let mut is_inside = false;
        let mut is_on_pipe = false;

        for column_index in 0..loop_map.tile_map.number_of_columns {
            let tile = AreaMap::get_tile(loop_map, index_offset + column_index);
            let is_part_of_loop = loop_map.contains_2d(row, column_index);

            if let Pipe(shape) = tile {
                if is_part_of_loop {
                    if shape == NorthToSouth {
                        is_inside = !is_inside;
                    }

                    if shape == SouthToEast {
                        is_on_pipe = true;
                    }

                    if is_on_pipe && shape == SouthToWest {
                        is_on_pipe = false;
                    }

                    if is_on_pipe && shape == NorthToWest {
                        is_on_pipe = false;
                        is_inside = !is_inside;
                    }

                    if shape == NorthToEast {
                        is_on_pipe = true;
                        is_inside = !is_inside;
                    }
                }
            }

            if is_inside && !is_on_pipe && !is_part_of_loop {
                tile_indices.push(index_offset + column_index);
            }
        }

        return tile_indices;
    }

    fn get_tile(loop_map: &LoopMap, tile_index: usize) -> Tile {
        if tile_index == loop_map.start_tile_index {
            return Pipe(loop_map.start_tile_shape);
        }

        return (&loop_map.tile_map.tiles[tile_index]).clone();
    }
}

#[cfg(test)]
mod tests {
    use crate::day10::area_map::AreaMap;
    use crate::day10::loop_map::LoopMap;
    use crate::day10::tile_map::TileMap;

    #[test]
    fn test_area_map_001() {
        let input = "\
...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";

        let tile_map = TileMap::parse(&input).unwrap();
        let loop_map = LoopMap::find_first(&tile_map, &12).unwrap();
        let sut = AreaMap::calculate(&loop_map);

        assert_eq!(sut.area_within, 4);
    }

    #[test]
    fn test_area_map_002() {
        let input = "\
..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
..........";

        let tile_map = TileMap::parse(&input).unwrap();
        let loop_map = LoopMap::find_first(&tile_map, &11).unwrap();
        let sut = AreaMap::calculate(&loop_map);

        assert_eq!(sut.area_within, 4);
    }

    #[test]
    fn test_area_map_003() {
        let input = "\
.F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";

        let tile_map = TileMap::parse(&input).unwrap();
        let loop_map = LoopMap::find_first(&tile_map, &tile_map.start_tile_indices[0]).unwrap();
        let sut = AreaMap::calculate(&loop_map);

        assert_eq!(sut.area_within, 8);
    }
}
