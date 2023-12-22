use crate::day11::image::Image;
use crate::day11::node::Pixel;

pub struct ExpandedUniverse<'a> {
    pub image: &'a Image,
    expansion_factor: usize,
    horizontal_offsets: Vec<usize>,
    vertical_offsets: Vec<usize>,
}

impl ExpandedUniverse<'_> {
    pub fn expand(image: &Image, factor: usize) -> ExpandedUniverse {
        return ExpandedUniverse {
            image,
            expansion_factor: factor,
            horizontal_offsets: ExpandedUniverse::calculate_horizontal_offsets(image, factor),
            vertical_offsets: ExpandedUniverse::calculate_vertical_offsets(image, factor),
        };
    }

    pub fn challenge1(image: &Image) -> ExpandedUniverse {
        return ExpandedUniverse::expand(image, 2);
    }

    pub fn challenge2(image: &Image) -> ExpandedUniverse {
        return ExpandedUniverse::expand(image, 1_000_000);
    }

    fn calculate_horizontal_offsets(image: &Image, expansion_factor: usize) -> Vec<usize> {
        let mut offsets = vec![0usize];

        for column in 0..image.number_of_columns - 1 {
            let last_offset = &offsets[offsets.len() - 1];
            if ExpandedUniverse::column_is_empty(image, column) {
                offsets.push(*last_offset + expansion_factor - 1);
            } else {
                offsets.push(*last_offset);
            }
        }

        return offsets;
    }

    fn column_is_empty(image: &Image, column: usize) -> bool {
        for row in 0..image.number_of_rows {
            let index = image.get_1d_index(row, column);
            if let Pixel::Galaxy = &image.pixels[index] {
                return false;
            }
        }

        return true;
    }

    fn calculate_vertical_offsets(image: &Image, expansion_factor: usize) -> Vec<usize> {
        let mut offsets = vec![0usize];

        for row in 0..image.number_of_rows - 1 {
            let last_offset = &offsets[offsets.len() - 1];
            if ExpandedUniverse::row_is_empty(image, row) {
                offsets.push(*last_offset + expansion_factor - 1);
            } else {
                offsets.push(*last_offset);
            }
        }

        return offsets;
    }

    fn row_is_empty(image: &Image, row: usize) -> bool {
        for column in 0..image.number_of_columns {
            let index = image.get_1d_index(row, column);
            if let Pixel::Galaxy = &image.pixels[index] {
                return false;
            }
        }

        return true;
    }

    pub fn get_2d_index(&self, index_1d: usize) -> (usize, usize) {
        let (initial_x, initial_y) = self.image.get_2d_index(index_1d);
        return (
            initial_x + &self.vertical_offsets[initial_x],
            initial_y + &self.horizontal_offsets[initial_y],
        );
    }
}

#[cfg(test)]
mod tests {
    use crate::day11::expanded_universe::ExpandedUniverse;
    use crate::day11::image::Image;

    #[test]
    fn test_challenge1_001() {
        let input = "\
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

        let image = Image::parse(&input).unwrap();
        let sut = ExpandedUniverse::challenge1(&image);

        assert_eq!(sut.horizontal_offsets.len(), image.number_of_columns);
        assert_eq!(sut.horizontal_offsets, vec![0, 0, 0, 1, 1, 1, 2, 2, 2, 3]);

        assert_eq!(sut.vertical_offsets.len(), image.number_of_rows);
        assert_eq!(sut.vertical_offsets, vec![0, 0, 0, 0, 1, 1, 1, 1, 2, 2]);
    }

    #[test]
    fn test_get_2d_index_challenge1_001() {
        let input = "\
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

        let image = Image::parse(&input).unwrap();
        let sut = ExpandedUniverse::challenge1(&image);

        assert_eq!(sut.get_2d_index(3), (0, 4));
        assert_eq!(sut.get_2d_index(17), (1, 9));
        assert_eq!(sut.get_2d_index(20), (2, 0));
        assert_eq!(sut.get_2d_index(46), (5, 8));
    }
}
