use std::collections::HashMap;

use crate::day8::direction::Direction;
use crate::string_functions::split_and_clean;
use crate::GenericError;

pub struct Map {
    pub directions: Vec<Direction>,
    pub start_indices: Vec<usize>,
    pub target_indices: Vec<usize>,
    pub nodes: Vec<Node>,
}

impl Map {
    pub fn parse(input: &str) -> Result<Map, GenericError> {
        let lines: Vec<&str> = input.lines().collect::<Vec<&str>>();
        if lines.len() < 3 {
            return Err(GenericError::new("no nodes in input"));
        }

        let directions = Direction::parse_all(lines[0])?;
        let raw_nodes = Node::parse_raw_nodes(&lines[2..])?;
        let name_lookup = Node::create_node_name_to_index_lookup(&raw_nodes);
        let nodes = Node::batch_create(&raw_nodes, &name_lookup)?;

        let start_node_names = name_lookup
            .keys()
            .filter(|k| k.ends_with("A"))
            .map(|k| *k)
            .collect::<Vec<&str>>();
        let end_node_names = name_lookup
            .keys()
            .filter(|k| k.ends_with("Z"))
            .map(|k| *k)
            .collect::<Vec<&str>>();

        let start_node_indices = Map::lookup_all(&start_node_names, &name_lookup)?;
        let end_node_indices = Map::lookup_all(&end_node_names, &name_lookup)?;

        return Ok(Map {
            directions,
            start_indices: start_node_indices,
            target_indices: end_node_indices,
            nodes,
        });
    }

    fn lookup_all(
        names: &Vec<&str>,
        lookup: &HashMap<&str, usize>,
    ) -> Result<Vec<usize>, GenericError> {
        let mut result: Vec<usize> = Vec::new();
        for name in names {
            let index = lookup.get(name);
            if index.is_none() {
                return Err(GenericError::new("unable to find index of node"));
            }

            result.push(*index.unwrap());
        }

        return Ok(result);
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Node {
    index: usize,
    left_child_index: usize,
    right_child_index: usize,
}

#[derive(Debug, Eq, PartialEq)]
struct RawNode<'a> {
    name: &'a str,
    left_child_name: &'a str,
    right_child_name: &'a str,
}

impl Node {
    pub fn parse_all(input: &[&str]) -> Result<(Vec<Node>, usize), GenericError> {
        let raw_nodes = Node::parse_raw_nodes(input)?;
        let name_lookup = Node::create_node_name_to_index_lookup(&raw_nodes);

        let nodes = Node::batch_create(&raw_nodes, &name_lookup)?;

        let target_node_index = name_lookup.get("ZZZ");
        if target_node_index.is_none() {
            return Err(GenericError::new("unable to find target node index"));
        }

        return Ok((nodes, *target_node_index.unwrap()));
    }

    fn batch_create(
        raw_nodes: &Vec<RawNode>,
        name_lookup: &HashMap<&str, usize>,
    ) -> Result<Vec<Node>, GenericError> {
        let mut result: Vec<Node> = Vec::new();
        for (index, raw_node) in raw_nodes.iter().enumerate() {
            let left_index = name_lookup.get(raw_node.left_child_name);
            let right_index = name_lookup.get(raw_node.right_child_name);

            if left_index.is_none() || right_index.is_none() {
                return Err(GenericError::new("unable to find child node index"));
            }

            result.push(Node {
                index,
                left_child_index: *left_index.unwrap(),
                right_child_index: *right_index.unwrap(),
            });
        }

        return Ok(result);
    }

    fn parse_raw_nodes<'a>(lines: &'a [&'a str]) -> Result<Vec<RawNode<'a>>, GenericError> {
        let mut result: Vec<RawNode> = Vec::new();
        for line in lines {
            result.push(Node::parse_raw_node(line)?);
        }

        return Ok(result);
    }

    fn parse_raw_node(input: &str) -> Result<RawNode, GenericError> {
        let equal_split = split_and_clean(input, "=");
        if equal_split.len() != 2 {
            return Err(GenericError::new(
                "unable to extract node name and child nodes",
            ));
        }

        let comma_split = split_and_clean(equal_split[1], ",");
        if comma_split.len() != 2 {
            return Err(GenericError::new(
                "unable to extract node name and child nodes",
            ));
        }

        return Ok(RawNode {
            name: equal_split[0],
            left_child_name: &comma_split[0][1..],
            right_child_name: &comma_split[1][..comma_split[1].len() - 1],
        });
    }

    fn create_node_name_to_index_lookup<'a>(
        raw_nodes: &Vec<RawNode<'a>>,
    ) -> HashMap<&'a str, usize> {
        let mut result: HashMap<&str, usize> = HashMap::new();
        for (index, node) in raw_nodes.iter().enumerate() {
            result.insert(node.name, index);
        }

        return result;
    }

    pub fn child_index(&self, direction: &Direction) -> usize {
        match direction {
            Direction::Left => self.left_child_index,
            Direction::Right => self.right_child_index,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::day8::node::{Node, RawNode};

    #[test]
    fn test_parse_all() {
        let input = "\
PGQ = (JQC, HNP)
JQC = (JQC, PGQ)
HNP = (JQC, ZZZ)
ZZZ = (ZZZ, ZZZ)"
            .lines()
            .collect::<Vec<&str>>();

        let actual = Node::parse_all(&input).unwrap().0;

        assert_eq!(
            actual,
            vec![
                Node {
                    index: 0,
                    left_child_index: 1,
                    right_child_index: 2,
                },
                Node {
                    index: 1,
                    left_child_index: 1,
                    right_child_index: 0,
                },
                Node {
                    index: 2,
                    left_child_index: 1,
                    right_child_index: 3,
                },
                Node {
                    index: 3,
                    left_child_index: 3,
                    right_child_index: 3,
                },
            ]
        );
    }

    #[test]
    fn test_parse_raw_nodes() {
        let input = "\
PGQ = (QRB, MJB)
JQC = (MNM, TLQ)
HNP = (NKD, PJT)
MDM = (SPC, RJP)"
            .lines()
            .collect::<Vec<&str>>();
        let actual = Node::parse_raw_nodes(&input).unwrap();

        assert_eq!(
            actual,
            vec![
                RawNode {
                    name: "PGQ",
                    left_child_name: "QRB",
                    right_child_name: "MJB",
                },
                RawNode {
                    name: "JQC",
                    left_child_name: "MNM",
                    right_child_name: "TLQ",
                },
                RawNode {
                    name: "HNP",
                    left_child_name: "NKD",
                    right_child_name: "PJT",
                },
                RawNode {
                    name: "MDM",
                    left_child_name: "SPC",
                    right_child_name: "RJP",
                },
            ]
        );
    }
}
