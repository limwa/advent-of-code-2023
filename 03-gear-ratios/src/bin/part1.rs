use std::{fs, io::{self, BufRead}};

use gear_ratios::*;

fn main() -> io::Result<()>{

    let file = fs::File::open("input.txt")?;
    let lines = io::BufReader::new(file).lines()
        .filter_map(|line| line.ok());

    let schematic = read_schematic(lines);

    let sum: u32 = schematic.part_numbers.iter()
        .filter(|part_number| {
            let symbol_cols = (part_number.cols.start - 1)..(part_number.cols.end + 1);
            let symbol_lines = (part_number.line - 1)..=(part_number.line + 1);
            
            schematic.symbols.iter().any(|symbol| {
                // If any symbol is adjacent to number
                symbol_cols.contains(&symbol.col) && symbol_lines.contains(&symbol.line)              
            })
        })
        .map(|part_number| part_number.value)
        .sum();

    println!("Sum: {}", sum);

    Ok(())
}