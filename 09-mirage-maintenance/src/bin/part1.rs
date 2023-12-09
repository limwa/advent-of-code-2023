use std::{fs, io::{self, BufRead}};
use mirage_maintenance::*;

fn main() -> io::Result<()> {
    let file = fs::File::open("input.txt")?;
    let lines = io::BufReader::new(file).lines()
        .filter_map(|line| line.ok());

    let readings = read_readings(lines);

    let sum: i64 = readings.histories.iter()
        .map(|history| extrapolate_next_value(&history.history))
        .sum();

    println!("Sum: {}", sum);

    Ok(())
}

fn extrapolate_next_value(history: &[i64]) -> i64 {
    let differences = history.iter()
        .zip(history.iter().skip(1))
        .map(|(&a, &b)| b - a)
        .collect::<Vec<_>>();

    if differences.iter().all(|&diff| diff == 0) {
        return *history.last().unwrap();
    }

    let next_value = extrapolate_next_value(&differences);
    history.last().unwrap() + next_value
}