use std::error::Error;
use std::fs;

use crate::day10::map::Map;
use crate::GenericError;

mod direction;
mod map;

pub fn day10_challenge1(file_path: &str) -> Result<usize, Box<dyn Error>> {
    let text = fs::read_to_string(file_path)?;
    let map = Map::parse(&text)?;

    if map.start_node_indices.len() != 1 {
        return Err(Box::new(GenericError::new(
            "found more than one start node",
        )));
    }

    let mut path = vec![map.start_node_indices[0]];
    let mut maybe_paths = map.get_loops_recursive(&mut path, true);
    if maybe_paths.is_none() {
        return Err(Box::new(GenericError::new("no cycles found")));
    }

    let paths = maybe_paths.unwrap();
    if paths.len() != 1 {
        return Err(Box::new(GenericError::new("found not exactly one path")));
    }

    return Ok((paths[0].len() as f64 * 0.5f64).ceil() as usize);
}
