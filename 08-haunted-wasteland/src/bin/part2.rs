use haunted_wasteland::*;
use std::{fs, io::{self, BufRead}};

fn count_steps_for_node(map: &Map, node: &Node) -> u64 {
    let mut current_node = node;
    let mut current_instruction_index = 0;
    let mut steps = 0;

    while !current_node.name.ends_with("Z") {
        let current_instruction = &map.instructions[current_instruction_index];
        // println!("{}: {:?} -> {}", steps, current_instruction, current_node.name);

        match current_instruction {
            Instruction::Left => {
                current_node = map.nodes.get(&current_node.left).unwrap();
            },
            Instruction::Right => {
                current_node = map.nodes.get(&current_node.right).unwrap();
            },
        };

        current_instruction_index = (current_instruction_index + 1) % map.instructions.len();
        steps += 1;
    }

    steps
}

fn lcm(a: u64, b: u64) -> u64 {
    let mut curr_a = a;
    let mut curr_b = b;

    while curr_a != curr_b {
        if curr_a < curr_b {
            curr_a += a;
        } else {
            curr_b += b;
        }
    }

    curr_a
}


fn main() -> io::Result<()> {
    let file = fs::File::open("input.txt")?;
    let lines = io::BufReader::new(file).lines()
        .filter_map(|line| line.ok());

    let map = read_map(lines);

    let steps = map.nodes.iter()
        .filter(|(node_name, _)| node_name.ends_with("A"))
        .map(|(_, node)| count_steps_for_node(&map, node))
        .reduce(|acc, steps| lcm(acc, steps));

    println!("Steps: {:?}", steps);

    Ok(())
}