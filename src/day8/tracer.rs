use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

use crate::day8::node::Map;

#[derive(Debug, Eq, PartialEq)]
pub struct Cycle {
    pub offset: u128,
    pub length: u128,
    pub offset_results: Vec<u128>,
    pub repeated_results: Vec<u128>,
}

impl Display for Cycle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "offset: {}; length: {}; #offset_result: {}; #repeated_result: {}", self.offset, self.length, self.offset_results.len(), self.repeated_results.len())
    }
}

impl Cycle {
    pub fn find_all(map: &Map) -> Vec<Cycle> {
        let mut result = Vec::new();
        for start_node_index in &map.start_indices {
            result.push(Cycle::find(map, *start_node_index));
        }

        return result;
    }

    fn find(map: &Map, start_node_index: usize) -> Cycle {
        let mut steps = 0u128;
        let mut next_node_index = start_node_index;
        let mut next_direction_index = 0usize;
        let mut visited_nodes: HashMap<(usize, usize), u128> = HashMap::new(); // (<node_index>, <direction_index>) -> number of steps until then

        let mut result_steps: Vec<u128> = Vec::new();

        loop {
            let key = (next_node_index, next_direction_index);
            let entry = visited_nodes.entry(key);

            if let Entry::Occupied(occupied) = entry {
                // we found a cycle
                let cycle_offset = occupied.get();
                let length = steps - cycle_offset;
                let mut offset_results: Vec<u128> = Vec::new();
                let mut repeated_results: Vec<u128> = Vec::new();

                for result_step in result_steps {
                    if result_step < *cycle_offset {
                        offset_results.push(result_step);
                    } else {
                        repeated_results.push(result_step - cycle_offset);
                    }
                }

                return Cycle {
                    offset: *cycle_offset,
                    length,
                    offset_results,
                    repeated_results,
                };
            }

            if map.target_indices.contains(&next_node_index) {
                result_steps.push(steps);
            }

            entry.or_insert(steps);
            steps += 1;
            next_node_index = map.nodes[next_node_index].child_index(&map.directions[next_direction_index]);
            next_direction_index = (next_direction_index + 1) % map.directions.len();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::day8::node::Map;
    use crate::day8::tracer::Cycle;

    #[test]
    fn test_find_cycle() {
        let input = "\
LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        let map = Map::parse(&input).unwrap();

        let first_cycle = Cycle::find(&map, 0);

        assert_eq!(first_cycle, Cycle {
            offset: 1,
            length: 2,
            offset_results: Vec::new(),
            repeated_results: vec![1],
        });

        let second_cycle = Cycle::find(&map, 3);
        assert_eq!(second_cycle, Cycle {
            offset: 1,
            length: 6,
            offset_results: Vec::new(),
            repeated_results: vec![2, 5],
        })
    }
}