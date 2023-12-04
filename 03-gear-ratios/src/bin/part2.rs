use std::{fs, io::{self, BufRead}, collections::HashMap};

use gear_ratios::*;

fn main() -> io::Result<()>{

    let file = fs::File::open("input.txt")?;
    let lines = io::BufReader::new(file).lines()
        .filter_map(|line| line.ok());

    let schematic = read_schematic(lines);

    let mut part_numbers_per_gear: HashMap<(i32, i32), Vec<u32>> = HashMap::new();

    schematic.part_numbers.iter()
        .flat_map(|part_number| {
            let symbol_cols = (part_number.cols.start - 1)..(part_number.cols.end + 1);
            let symbol_lines = (part_number.line - 1)..=(part_number.line + 1);
            
            schematic.symbols.iter()
                .filter(move |symbol| {
                    // Get potential adjacent gears
                    symbol.character == "*"
                        && symbol_cols.contains(&symbol.col)
                        && symbol_lines.contains(&symbol.line)              
                })
                .map(|symbol| (symbol.col, symbol.line, part_number.value))
        })
        .for_each(|(gear_col, gear_line, part_number)| {
            let existing_part_numbers = part_numbers_per_gear.entry((gear_col, gear_line));
            existing_part_numbers.or_insert(Vec::new()).push(part_number);
        });

    let sum: u32 = part_numbers_per_gear.iter()
        .filter(|(_key, value)| value.len() == 2)
        .map(|(_key, value)| value.iter().fold(1, |acc, val| acc * val))
        .sum();


    println!("Sum: {}", sum);

    Ok(())
}