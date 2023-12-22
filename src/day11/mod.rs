use crate::day11::expanded_universe::ExpandedUniverse;
use crate::day11::image::Image;
use std::cmp::{max, min};
use std::fs;

mod expanded_universe;
mod image;
mod node;

pub fn solve_day11(file_path: &str) {
    let text = fs::read_to_string(file_path).expect("given challenge file cannot be read");
    let image = Image::parse(&text).expect("the given input should be a valid image");
    let expanded_universe = ExpandedUniverse::challenge1(&image);

    println!(
        "Day11 - Challenge1: {}",
        sum_of_pairwise_distances(&expanded_universe)
    );
}

fn sum_of_pairwise_distances(universe: &ExpandedUniverse) -> u128 {
    let mut result = 0u128;
    for (i, galaxy_index_a) in universe.image.galaxy_indices.iter().enumerate() {
        for j in i..universe.image.galaxy_indices.len() {
            let galaxy_index_b = &universe.image.galaxy_indices[j];

            let (x_a, y_a) = universe.get_2d_index(*galaxy_index_a);
            let (x_b, y_b) = universe.get_2d_index(*galaxy_index_b);
            let diff_x = max(x_a, x_b) - min(x_a, x_b);
            let diff_y = max(y_a, y_b) - min(y_a, y_b);

            result += (diff_x + diff_y) as u128;
        }
    }

    return result;
}
