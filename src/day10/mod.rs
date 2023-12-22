use std::fs;

use crate::day10::area_map::AreaMap;
use crate::day10::loop_map::LoopMap;
use crate::day10::tile_map::TileMap;

mod area_map;
mod direction;
mod loop_map;
mod tile;
mod tile_map;

pub fn solve_day10(file_path: &str) {
    let text = fs::read_to_string(file_path).expect("given challenge file cannot be read");
    let tile_map = TileMap::parse(&text).expect("cannot parse input data");

    if tile_map.start_tile_indices.len() != 1 {
        panic!("tile map doesn't contain exactly one start tile");
    }

    let loop_map = LoopMap::find_first(&tile_map, &tile_map.start_tile_indices[0])
        .expect("no loop in tile map found");

    let half_length = (loop_map.length as f64 * 0.5f64).ceil() as i128;
    println!("Day10 - Challenge1: {half_length}");

    let area_map = AreaMap::calculate(&loop_map);
    println!("Day10 - Challenge2: {}", area_map.area_within);
}
