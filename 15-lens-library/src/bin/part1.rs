use std::{fs, io::{self, BufRead}};
use lens_library::*;

fn main() -> io::Result<()> {
    let file = fs::File::open("input.txt")?;
    let lines = io::BufReader::new(file).lines()
        .filter_map(|line| line.ok());

    let steps = read_steps(lines);
    let sum = steps.iter()
        .map(|step| holiday_ascii_helper(&step) as u32)
        .sum::<u32>();

    println!("Sum: {}", sum);

    Ok(())
}