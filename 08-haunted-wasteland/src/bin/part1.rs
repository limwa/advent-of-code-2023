use haunted_wasteland::*;
use std::{fs, io::{self, BufRead}};

fn main() -> io::Result<()> {
    let file = fs::File::open("input.txt")?;
    let lines = io::BufReader::new(file).lines()
        .filter_map(|line| line.ok());

    let map = read_map(lines);

    let mut current_node = map.nodes.get("AAA").unwrap();
    let mut current_instruction_index = 0;
    let mut steps = 0;

    while current_node.name != "ZZZ" {
        let current_instruction = &map.instructions[current_instruction_index];
        println!("{}: {:?} -> {}", steps, current_instruction, current_node.name);

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

    println!("Steps: {}", steps);

    Ok(())
}