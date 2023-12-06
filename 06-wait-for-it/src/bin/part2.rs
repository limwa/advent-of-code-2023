use wait_for_it::*;
use std::{fs, io::{self, BufRead}};

fn main() -> io::Result<()> {
    let file = fs::File::open("input.txt")?;
    let lines = io::BufReader::new(file).lines()
        .filter_map(|line| line.ok())
        .map(|line| line.replace(" ", ""));

    let races = read_races(lines);

    let prod = races.iter()
        .map(|race| find_possible_charging_times(race))
        .map(|range| range.size_hint())
        .map(|(lower, upper)| {
            if let Some(upper) = upper {
                if lower == upper {
                    return lower;
                }

                panic!("Different values for upper and lower: {} and {}", lower, upper);
            }

            panic!("No value for upper, only lower: {}", lower);
        })
        .fold(1, |acc, val| acc * val);
    
    println!("Value: {}", prod);

    Ok(())
}
