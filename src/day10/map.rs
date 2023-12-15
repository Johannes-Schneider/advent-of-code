use crate::day10::direction::Direction;
use crate::day10::direction::Direction::{East, North, South, West};
use crate::GenericError;

pub struct Map {
    nodes: Vec<MapNode>,
    columns: usize,
    rows: usize,
    pub start_node_indices: Vec<usize>,
}

impl Map {
    pub fn parse(input: &str) -> Result<Map, GenericError> {
        let tiles = RawTile::parse_input(input)?;
        let columns = Map::number_of_columns(&tiles)?;
        let rows = tiles.len();
        let nodes = Map::convert_tiles_to_nodes(&tiles, columns);
        let start_node_indices = Map::extract_start_node_indices(&nodes)?;

        return Ok(Map {
            nodes,
            columns,
            rows,
            start_node_indices,
        });
    }

    fn number_of_columns(tiles: &Vec<Vec<RawTile>>) -> Result<usize, GenericError> {
        if tiles.len() < 1 {
            return Ok(0);
        }

        let columns = tiles[0].len();
        for row in &tiles[1..] {
            if row.len() != columns {
                return Err(GenericError::new(
                    "all rows must have the same amount of columns",
                ));
            }
        }

        return Ok(columns);
    }

    fn convert_tiles_to_nodes(tiles: &Vec<Vec<RawTile>>, columns: usize) -> Vec<MapNode> {
        let mut result = Vec::new();
        for (row_index, row) in tiles.iter().enumerate() {
            for (column_index, tile) in row.iter().enumerate() {
                let mut node = MapNode::from_raw_tile(tile);

                if column_index > 0
                    && tile.is_connected_to(&tiles[row_index][column_index - 1], &West)
                {
                    let neighbor_index = Map::to_1d_index(row_index, column_index - 1, columns);
                    let my_index = Map::to_1d_index(row_index, column_index, columns);
                    let neighbor: &mut MapNode = result.get_mut(neighbor_index).unwrap();
                    node.add_neighbor(neighbor_index);
                    neighbor.add_neighbor(my_index);
                }

                if row_index > 0
                    && tile.is_connected_to(&tiles[row_index - 1][column_index], &North)
                {
                    let neighbor_index = Map::to_1d_index(row_index - 1, column_index, columns);
                    let my_index = Map::to_1d_index(row_index, column_index, columns);
                    let neighbor = result.get_mut(neighbor_index).unwrap();
                    node.add_neighbor(neighbor_index);
                    neighbor.add_neighbor(my_index);
                }

                result.push(node);
            }
        }

        return result;
    }

    fn extract_start_node_indices(nodes: &Vec<MapNode>) -> Result<Vec<usize>, GenericError> {
        let mut result = Vec::new();
        for (i, n) in nodes.iter().enumerate() {
            if n.is_start {
                result.push(i);
            }
        }

        if result.is_empty() {
            return Err(GenericError::new("no start nodes found"));
        }

        return Ok(result);
    }

    fn to_1d_index(row_index: usize, column_index: usize, columns: usize) -> usize {
        row_index * columns + column_index
    }

    pub fn get_loops_recursive(
        &self,
        path: &mut Vec<usize>,
        stop_after_first: bool,
    ) -> Option<Vec<Vec<usize>>> {
        if path.is_empty() {
            panic!("the loop path must never be empty");
        }

        let last_node_index = &path[path.len() - 1];
        let last_node = &self.nodes[*last_node_index];

        let mut excluded_next_nodes = vec![last_node_index];
        if path.len() >= 2 {
            excluded_next_nodes.push(&path[path.len() - 2]);
        }

        let next_node_indices = last_node
            .neighbor_indices
            .iter()
            .filter(|i| !excluded_next_nodes.contains(i))
            .collect::<Vec<&usize>>();

        if next_node_indices.is_empty() {
            // there is no path
            return None;
        }

        if next_node_indices.len() == 1 {
            if next_node_indices[0] == &path[0] {
                return Some(vec![path.to_vec()]);
            }

            // we are continuing on a single path, so we can re-use the existing vec instance
            path.push(*next_node_indices[0]);
            return self.get_loops_recursive(path, stop_after_first);
        }

        let mut result = Vec::new();
        for next_node_index in next_node_indices {
            if next_node_index == &path[0] {
                result.push(path.to_vec());
                continue;
            }

            let mut new_path = path.to_vec();
            new_path.push(*next_node_index);
            let loops = self.get_loops_recursive(&mut new_path, stop_after_first);
            if loops.is_none() {
                continue;
            }

            for l in loops.unwrap() {
                result.push(l);

                if stop_after_first {
                    return Some(result);
                }
            }
        }

        if result.is_empty() {
            return None;
        }

        return Some(result);
    }
}

struct MapNode {
    neighbor_indices: Vec<usize>,
    is_start: bool,
}

impl MapNode {
    fn from_raw_tile(tile: &RawTile) -> MapNode {
        return MapNode {
            neighbor_indices: Vec::new(),
            is_start: tile.is_start,
        };
    }

    fn add_neighbor(&mut self, index: usize) {
        self.neighbor_indices.push(index)
    }
}

struct RawTile {
    row_index: usize,
    column_index: usize,
    directions: Vec<Direction>,
    is_start: bool,
}

impl RawTile {
    fn parse_input(input: &str) -> Result<Vec<Vec<RawTile>>, GenericError> {
        let mut result = Vec::new();
        for (row_index, line) in input.lines().enumerate() {
            result.push(RawTile::parse_line(row_index, line)?);
        }

        return Ok(result);
    }

    fn parse_line(row_index: usize, line: &str) -> Result<Vec<RawTile>, GenericError> {
        let mut result = Vec::new();
        for (column_index, byte) in line.as_bytes().iter().enumerate() {
            result.push(RawTile::parse(row_index, column_index, byte)?);
        }

        return Ok(result);
    }

    fn parse(row_index: usize, column_index: usize, input: &u8) -> Result<RawTile, GenericError> {
        match input {
            b'|' => Ok(RawTile::pipe(row_index, column_index, vec![North, South])),
            b'-' => Ok(RawTile::pipe(row_index, column_index, vec![East, West])),
            b'L' => Ok(RawTile::pipe(row_index, column_index, vec![North, East])),
            b'J' => Ok(RawTile::pipe(row_index, column_index, vec![North, West])),
            b'7' => Ok(RawTile::pipe(row_index, column_index, vec![South, West])),
            b'F' => Ok(RawTile::pipe(row_index, column_index, vec![South, East])),
            b'.' => Ok(RawTile::ground(row_index, column_index)),
            b'S' => Ok(RawTile::start(row_index, column_index)),
            _ => Err(GenericError::new("unknown tile type")),
        }
    }

    fn pipe(row_index: usize, column_index: usize, directions: Vec<Direction>) -> RawTile {
        return RawTile {
            row_index,
            column_index,
            directions,
            is_start: false,
        };
    }

    fn ground(row_index: usize, column_index: usize) -> RawTile {
        return RawTile {
            row_index,
            column_index,
            directions: Vec::new(),
            is_start: false,
        };
    }

    fn start(row_index: usize, column_index: usize) -> RawTile {
        return RawTile {
            row_index,
            column_index,
            directions: vec![North, East, South, West],
            is_start: true,
        };
    }

    fn is_connected_to(&self, other_tile: &RawTile, relative_direction: &Direction) -> bool {
        if self.directions.is_empty() || other_tile.directions.is_empty() {
            return false;
        }

        if !self.directions.contains(relative_direction) {
            return false;
        }

        return other_tile
            .directions
            .contains(&relative_direction.inverse());
    }
}

#[cfg(test)]
mod tests {
    use crate::day10::map::Map;

    #[test]
    fn integration_test() {
        let input = "\
.....
.S-7.
.|.|.
.L-J.
.....";
        let map = Map::parse(&input).unwrap();
        let mut path = vec![6];

        assert_eq!(map.start_node_indices, vec![6]);
        assert_eq!(
            map.get_loops_recursive(&mut path, true),
            Some(vec![vec![6, 7, 8, 13, 18, 17, 16, 11],])
        );
    }
}
