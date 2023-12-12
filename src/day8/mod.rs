use crate::day8::direction::Direction;
use crate::day8::node::Node;
use crate::GenericError;
use std::error::Error;
use std::fs;

mod direction;
mod node;

pub fn day8_challenge1_naive(file_path: &str) -> Result<u128, Box<dyn Error>> {
    let text = fs::read_to_string(file_path)?;
    let (direction, nodes, target_node_index) = parse_input(&text)?;

    let mut next_node_index = 0usize;
    let mut next_direction_index = 0usize;
    let mut steps = 0u128;
    while next_node_index != target_node_index {
        next_node_index = nodes[next_node_index].child_index(&direction[next_direction_index]);
        next_direction_index = (next_direction_index + 1) % direction.len();
        steps += 1;
    }

    return Ok(steps);
}

fn parse_input(input: &str) -> Result<(Vec<Direction>, Vec<Node>, usize), GenericError> {
    let lines: Vec<&str> = input.lines().collect::<Vec<&str>>();
    if lines.len() < 3 {
        return Err(GenericError::new("no nodes in input"));
    }

    let directions = Direction::parse_all(lines[0])?;
    let (nodes, target_node_index) = Node::parse_all(&lines[2..])?;

    return Ok((directions, nodes, target_node_index));
}
