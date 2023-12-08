use std::collections::HashMap;

use regex::Regex;

#[derive(Clone, Debug)]
pub struct Node {
    pub name: String,
    pub left: String,
    pub right: String,
}

#[derive(Debug)]
pub enum Instruction {
    Left,
    Right,
}

#[derive(Default, Debug)]
pub struct Map {
    pub instructions: Vec<Instruction>,
    pub nodes: HashMap<String, Node>,
}

pub fn read_map<'a>(mut reader: impl Iterator<Item = String>) -> Map {
    let mut map = Map::default();
    
    let instructions_str = reader.next().unwrap();
    instructions_str.chars()
        .map(|c| match c {
            'L' => Instruction::Left,
            'R' => Instruction::Right,
            _ => panic!("Invalid instruction: {}", c),
        })
        .for_each(|instr| map.instructions.push(instr));

    reader.next(); // Skip empty line

    let nodes_regex = Regex::new(r"(\w+) = \((\w+), (\w+)\)").unwrap();

    reader.map(|line| {
        let capture = nodes_regex.captures(&line).unwrap();

        let node_name = capture.get(1)
            .map(|group| group.as_str())
            .map(|str| str.to_string())
            .unwrap();
        
        let left_name = capture.get(2)
            .map(|group| group.as_str())
            .map(|str| str.to_string())
            .unwrap();

        let right_name = capture.get(3)
            .map(|group| group.as_str())
            .map(|str| str.to_string())
            .unwrap();

        Node {
            name: node_name,
            left: left_name,
            right: right_name,
        }
    })
    .for_each(|node| {
        let node_name = node.name.clone();
        map.nodes.insert(node_name, node);
    });

    map
}