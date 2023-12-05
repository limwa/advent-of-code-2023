use std::{fs, io::{self, BufRead}};

use if_you_give_a_seed_a_fertilizer::*;

fn main() -> io::Result<()> {
    let file = fs::File::open("input.txt")?;
    let lines = io::BufReader::new(file).lines()
        .filter_map(|line| line.ok());

    let almanac = read_almanac(lines);
    let mut mapped_values = Vec::new();

    for seed in almanac.seeds {
        let mut current_value = seed;

        for category in &almanac.mappings {
            for range in &category.ranges {
                if current_value >= range.source_start && current_value < range.source_start + range.size {
                    current_value = current_value - range.source_start + range.destination_start;
                    break;
                }
            }

        }
        
        mapped_values.push(current_value);
    }

    println!("Value: {:?}", mapped_values.iter().min());

    Ok(())
}