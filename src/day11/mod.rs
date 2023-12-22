use std::cmp::{max, min};
use std::fs;

use crate::day11::expanded_universe::ExpandedUniverse;
use crate::day11::image::Image;

mod expanded_universe;
mod image;
mod node;

pub fn solve_day11(file_path: &str) {
    let text = fs::read_to_string(file_path).expect("given challenge file cannot be read");
    let image = Image::parse(&text).expect("the given input should be a valid image");
    let universe_challenge1 = ExpandedUniverse::challenge1(&image);

    println!(
        "Day11 - Challenge1: {}",
        sum_of_pairwise_distances(&universe_challenge1)
    );

    let universe_challenge2 = ExpandedUniverse::challenge2(&image);
    println!(
        "Day11 - Challenge2: {}",
        sum_of_pairwise_distances(&universe_challenge2)
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

#[cfg(test)]
mod tests {
    use crate::day11::expanded_universe::ExpandedUniverse;
    use crate::day11::image::Image;
    use crate::day11::sum_of_pairwise_distances;

    static INPUT: &'static str = "\
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    #[test]
    fn test_sum_of_pairwise_distances_expansion_2() {
        let image = Image::parse(&INPUT).unwrap();
        let universe = ExpandedUniverse::expand(&image, 2);

        let actual = sum_of_pairwise_distances(&universe);

        assert_eq!(actual, 374);
    }

    #[test]
    fn test_sum_of_pairwise_distances_expansion_10() {
        let image = Image::parse(&INPUT).unwrap();
        let universe = ExpandedUniverse::expand(&image, 10);

        let actual = sum_of_pairwise_distances(&universe);

        assert_eq!(actual, 1030);
    }

    #[test]
    fn test_sum_of_pairwise_distances_expansion_100() {
        let image = Image::parse(&INPUT).unwrap();
        let universe = ExpandedUniverse::expand(&image, 100);

        let actual = sum_of_pairwise_distances(&universe);

        assert_eq!(actual, 8410);
    }
}
