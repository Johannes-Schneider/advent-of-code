use crate::day10::direction::Direction;
use crate::day10::tile::Tile;
use crate::GenericError;

pub struct TileMap {
    pub tiles: Vec<Tile>,
    pub number_of_columns: usize,
    pub number_of_rows: usize,
    pub start_tile_indices: Vec<usize>,
    pub neighbor_indices: Vec<Vec<usize>>,
}

impl TileMap {
    pub fn parse(input: &str) -> Result<TileMap, GenericError> {
        let (tiles, number_of_columns, number_of_rows) = TileMap::parse_tiles(input)?;
        let start_tile_indices = TileMap::get_start_tile_indices(&tiles)?;
        let neighbor_indices =
            TileMap::get_neighbor_indices(&tiles, number_of_columns, number_of_rows);

        return Ok(TileMap {
            tiles,
            number_of_columns,
            number_of_rows,
            start_tile_indices,
            neighbor_indices,
        });
    }

    fn parse_tiles(input: &str) -> Result<(Vec<Tile>, usize, usize), GenericError> {
        let mut tiles = Vec::new();
        let mut number_of_columns = None;
        let mut number_of_rows = 0usize;

        for line in input.lines() {
            let mut row = TileMap::parse_row(line)?;
            if number_of_columns.is_none() {
                number_of_columns = Some(row.len());
            }

            if row.len() != number_of_columns.unwrap_or(0) {
                return Err(GenericError::new("all rows must be equally long"));
            }

            tiles.append(&mut row);
            number_of_rows += 1;
        }

        return Ok((tiles, number_of_columns.unwrap_or(0), number_of_rows));
    }

    fn parse_row(input: &str) -> Result<Vec<Tile>, GenericError> {
        let mut result = Vec::new();
        for byte in input.as_bytes() {
            result.push(Tile::parse(byte)?);
        }

        return Ok(result);
    }

    fn get_start_tile_indices(tiles: &Vec<Tile>) -> Result<Vec<usize>, GenericError> {
        let mut start_tile_indices = Vec::new();
        for (index, tile) in tiles.iter().enumerate() {
            if let Tile::Start = tile {
                start_tile_indices.push(index);
            }
        }

        if start_tile_indices.is_empty() {
            return Err(GenericError::new("tile map does not have any start tiles"));
        }

        return Ok(start_tile_indices);
    }

    fn get_neighbor_indices(
        tiles: &Vec<Tile>,
        number_of_columns: usize,
        number_of_rows: usize,
    ) -> Vec<Vec<usize>> {
        let mut neighbor_indices = Vec::new();

        for row_index in 0..number_of_rows {
            for column_index in 0..number_of_columns {
                let mut current_neighbor_indices = Vec::new();

                let current_index =
                    TileMap::get_1d_index(row_index, column_index, number_of_columns);
                let current_tile = &tiles[current_index];

                if column_index > 0 {
                    let left_index =
                        TileMap::get_1d_index(row_index, column_index - 1, number_of_columns);
                    let neighbor = &tiles[left_index];

                    if current_tile.can_reach(neighbor, &Direction::West) {
                        let left_neighbor_indices: &mut Vec<usize> =
                            neighbor_indices.get_mut(left_index).unwrap();
                        left_neighbor_indices.push(current_index);
                        current_neighbor_indices.push(left_index);
                    }
                }

                if row_index > 0 {
                    let top_index =
                        TileMap::get_1d_index(row_index - 1, column_index, number_of_columns);
                    let neighbor = &tiles[top_index];

                    if current_tile.can_reach(neighbor, &Direction::North) {
                        let top_neighbor_indices: &mut Vec<usize> =
                            neighbor_indices.get_mut(top_index).unwrap();
                        top_neighbor_indices.push(current_index);
                        current_neighbor_indices.push(top_index);
                    }
                }

                neighbor_indices.push(current_neighbor_indices);
            }
        }

        return neighbor_indices;
    }

    fn get_1d_index(row_index: usize, column_index: usize, number_of_columns: usize) -> usize {
        return row_index * number_of_columns + column_index;
    }
}

#[cfg(test)]
mod tests {
    use crate::day10::tile_map::TileMap;

    #[test]
    fn test_parse_001() {
        let input = "\
.....
.S-7.
.|.|.
.L-J.
.....";
        let actual = TileMap::parse(&input).unwrap();

        assert_eq!(actual.number_of_columns, 5);
        assert_eq!(actual.number_of_rows, 5);
        assert_eq!(actual.tiles.len(), 25);
        assert_eq!(actual.neighbor_indices.len(), 25);

        assert_eq!(actual.neighbor_indices[6], vec![7, 11]);
    }
}
