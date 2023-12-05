use std::{fs, io::{self, BufRead}};

use scratchcards::*;

fn main() -> io::Result<()> {
    
    let file = fs::File::open("input.txt")?;
    let lines = io::BufReader::new(file).lines()
        .filter_map(|line| line.ok());

    let scratchcards = read_pile_of_scratchcards(lines);

    let sum: i32 = scratchcards.iter()
        .map(|scratchcard| {

            scratchcard.played_numbers.iter()
                .filter(|played_number| scratchcard.winning_numbers.contains(played_number))
                .count()
        })
        .filter(|&number_of_winning_numbers| number_of_winning_numbers > 0)
        .map(|number_of_winning_numbers| 2_i32.pow(number_of_winning_numbers as u32 - 1))
        .sum();

    println!("Sum: {}", sum);

    Ok(())
}