use std::{fs, io::{self, BufRead}};
use lens_library::*;
use linked_hash_map::LinkedHashMap;
use regex::Regex;

struct Operation {
    label: String,
    op_type: OperationType,
}

enum OperationType {
    Insert(u8),
    Remove,
}

fn main() -> io::Result<()> {
    let file = fs::File::open("input.txt")?;
    let lines = io::BufReader::new(file).lines()
        .filter_map(|line| line.ok());

    let step_regex = Regex::new(r"^(?P<label>\w+)(?P<operation>-|=\d+)$").unwrap();

    let operations = read_steps(lines)
        .iter()
        .map(|step| {
            let captures = step_regex.captures(step).unwrap();
            let label = captures.name("label").unwrap().as_str().to_string();
            let operation = captures.name("operation").unwrap().as_str();

            Operation {
                label,
                op_type: match operation {
                    "-" => OperationType::Remove,
                    _ => {
                        let value = operation[1..].parse::<u8>().unwrap();
                        OperationType::Insert(value)
                    }
                }
            }
        })
        .collect::<Vec<_>>();
    
    let mut boxes = Vec::<LinkedHashMap<String, u8>>::with_capacity(256);
    for _ in 0..256 {
        boxes.push(LinkedHashMap::new());
    }

    for operation in operations {
        let hash = holiday_ascii_helper(&operation.label) as usize;
        let lens_box = boxes.get_mut(hash).unwrap();

        match operation.op_type {
            OperationType::Insert(value) => {
                let entry = lens_box.entry(operation.label).or_insert(value);
                *entry = value;
            },
            OperationType::Remove => {
                lens_box.remove(&operation.label);
            }
        }        
    }

    let sum = boxes.iter()
        .enumerate()
        .filter(|(_, lens_box)| !lens_box.is_empty())
        .flat_map(|(box_index, lens_box)| {
            lens_box.values()
                .enumerate()
                .map(move |(lens_index, &value)| (box_index + 1) * (lens_index + 1) * value as usize)
        })
        .sum::<usize>();

    println!("Sum: {}", sum);

    Ok(())
}