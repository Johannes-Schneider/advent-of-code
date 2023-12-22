use crate::day11::node::Pixel;
use crate::GenericError;

pub struct Image {
    pub pixels: Vec<Pixel>,
    pub number_of_rows: usize,
    pub number_of_columns: usize,
    pub galaxy_indices: Vec<usize>,
}

impl Image {
    pub fn parse(input: &str) -> Result<Image, GenericError> {
        let (pixels, number_of_rows, number_of_columns) = Image::parse_pixels(input)?;
        let galaxy_indices = Image::extract_galaxy_indices(&pixels);

        return Ok(Image {
            pixels,
            number_of_rows,
            number_of_columns,
            galaxy_indices,
        });
    }

    fn parse_pixels(input: &str) -> Result<(Vec<Pixel>, usize, usize), GenericError> {
        let mut pixels = Vec::new();
        let mut number_of_rows = 0usize;
        let mut number_of_columns = None;

        for line in input.lines() {
            number_of_rows += 1;
            let mut row = Vec::new();

            for byte in line.as_bytes() {
                row.push(Pixel::parse(byte)?);
            }

            if number_of_columns.is_none() {
                number_of_columns = Some(row.len());
            }

            if row.len() != number_of_columns.unwrap_or(0) {
                return Err(GenericError::new("all rows must be equally long"));
            }

            pixels.append(&mut row);
        }

        return Ok((pixels, number_of_rows, number_of_columns.unwrap_or(0)));
    }

    fn extract_galaxy_indices(pixels: &Vec<Pixel>) -> Vec<usize> {
        let mut result = Vec::new();
        for (index, pixel) in pixels.iter().enumerate() {
            if let Pixel::Galaxy = pixel {
                result.push(index);
            }
        }

        return result;
    }

    pub fn get_1d_index(&self, row: usize, column: usize) -> usize {
        if row >= self.number_of_rows || column >= self.number_of_columns {
            panic!("index out of range");
        }

        return self.number_of_columns * row + column;
    }

    pub fn get_2d_index(&self, index_1d: usize) -> (usize, usize) {
        let column = index_1d % self.number_of_columns;
        let row = (index_1d - column) / self.number_of_columns;

        return (row, column);
    }
}

#[cfg(test)]
mod tests {
    use crate::day11::image::Image;

    #[test]
    fn test_get_2d_index_001() {
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
        let sut = Image::parse(&input).unwrap();

        assert_eq!(sut.get_2d_index(0), (0, 0));
        assert_eq!(sut.get_2d_index(9), (0, 9));
        assert_eq!(sut.get_2d_index(10), (1, 0));
        assert_eq!(sut.get_2d_index(19), (1, 9));
    }
}
