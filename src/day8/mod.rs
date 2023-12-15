use std::collections::HashMap;
use std::error::Error;
use std::fs;

use crate::day8::direction::Direction;
use crate::day8::node::{Map, Node};
use crate::day8::prime::{Prime, PrimeFactor};
use crate::day8::tracer::Cycle;
use crate::GenericError;

mod direction;
mod node;
mod prime;
mod tracer;

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

pub fn day8_challenge2_cycles(file_path: &str) -> Result<u128, Box<dyn Error>> {
    let text = fs::read_to_string(file_path)?;
    let map = Map::parse(&text)?;
    let cycles = Cycle::find_all(&map);

    for cycle in &cycles {
        // checking assumptions for challenge
        if cycle.repeated_results.len() != 1 {
            return Err(Box::new(GenericError::new(
                "found more than one result per cycle",
            )));
        }

        if cycle.offset_results.len() != 0 {
            return Err(Box::new(GenericError::new("found an offset result")));
        }

        if cycle.repeated_results[0] + cycle.offset != cycle.length {
            return Err(Box::new(GenericError::new(
                "initial target offset + cycle offset != cycle length",
            )));
        }
    }

    let mut prime = Prime::new();
    let mut prime_factors = Vec::new();
    for cycle in &cycles {
        prime_factors.push(prime.prime_factors(cycle.length));
    }

    return Ok(least_common_multiple(&prime_factors));
}

fn least_common_multiple(all_factors: &Vec<Vec<PrimeFactor>>) -> u128 {
    let mut exponents: HashMap<u128, u128> = HashMap::new();
    for factors in all_factors {
        for factor in factors {
            let maybe_existing_exponent = exponents.get(&factor.base);
            if *maybe_existing_exponent.unwrap_or(&0) < factor.exponent {
                exponents.insert(factor.base, factor.exponent);
            }
        }
    }

    let mut result = 1u128;
    for (k, v) in exponents {
        result *= k.pow(v as u32);
    }

    return result;
}