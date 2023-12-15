use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::hash::Hash;

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

pub fn day8_challenge2_naive(file_path: &str) -> Result<u128, Box<dyn Error>> {
    let text = fs::read_to_string(file_path)?;
    let map = Map::parse(&text)?;

    let mut next_node_indices = map.start_indices.to_vec();
    let mut next_direction_index = 0usize;
    let mut steps = 0u128;
    while next_node_indices != map.target_indices {
        for index in 0..next_node_indices.len() {
            let node_index = next_node_indices[index];
            next_node_indices[index] =
                map.nodes[node_index].child_index(&map.directions[next_direction_index]);
        }

        next_direction_index = (next_direction_index + 1) % map.directions.len();
        steps += 1;
    }

    return Ok(steps);
}

pub fn day8_challenge2_cycles(file_path: &str) -> Result<u128, Box<dyn Error>> {
    let text = fs::read_to_string(file_path)?;
    let map = Map::parse(&text)?;
    let cycles = Cycle::find_all(&map);

    for cycle in &cycles {
        // checking assumptions for challenge
        if cycle.repeated_results.len() != 1 {
            return Err(Box::new(GenericError::new("found more than one result per cycle")));
        }

        if cycle.offset_results.len() != 0 {
            return Err(Box::new(GenericError::new("found an offset result")));
        }

        if cycle.repeated_results[0] + cycle.offset != cycle.length {
            return Err(Box::new(GenericError::new("initial target offset + cycle offset != cycle length")));
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

fn check_if_offset_results_overlap(cycle: &Vec<Cycle>) -> Option<u128> {
    if cycle.is_empty() {
        return None;
    }

    for steps in &cycle[0].offset_results {
        if cycle[1..]
            .iter()
            .all(|c| c.offset_results.iter().any(|s| s == steps))
        {
            return Some(*steps);
        }
    }

    return None;
}

fn brute_force_steps(cycles: &Vec<Cycle>) -> u128 {
    let mut single_result_cycle = Vec::new();
    for cycle in cycles {
        for steps in &cycle.repeated_results {
            single_result_cycle.push(Cycle {
                offset: cycle.offset,
                offset_results: Vec::new(),
                length: cycle.length,
                repeated_results: vec![*steps],
            });
        }
    }

    let mut steps = single_result_cycle
        .iter()
        .map(|c| steps_after_loops(c, 0))
        .collect::<Vec<u128>>();
    let mut loops = single_result_cycle
        .iter()
        .map(|_| 0u128)
        .collect::<Vec<u128>>();
    while !all_elements_equal(&steps) {
        let mut min_steps = u128::MAX;
        let mut min_index = usize::MAX;
        let mut max_steps = u128::MIN;
        let mut max_index = usize::MIN;

        for (index, s) in steps.iter().enumerate() {
            if *s < min_steps {
                min_steps = *s;
                min_index = index;
            }

            if *s > max_steps {
                max_steps = *s;
                max_index = index;
            }
        }

        println!("max: {max_steps} vs. min: {min_steps}");

        let mut l = loops[min_index] + 1;
        let mut s = steps_after_loops(&single_result_cycle[min_index], l);
        while s < max_steps {
            l += 1;
            s = steps_after_loops(&single_result_cycle[min_index], l);
        }

        loops[min_index] = l;
        steps[min_index] = s;
    }

    return steps[0];
}

fn steps_after_loops(cycle: &Cycle, loops: u128) -> u128 {
    cycle.offset + cycle.repeated_results[0] + loops * cycle.length
}

fn all_elements_equal(input: &Vec<u128>) -> bool {
    input.iter().all(|e| *e == input[0])
}
