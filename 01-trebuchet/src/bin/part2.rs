use std::fs::File;
use std::io::{BufRead, self};

use regex::Regex;

fn main() -> std::io::Result<()> {

    let file = File::open("input.txt")?;
    let reader = io::BufReader::new(file);

    let first_regex = Regex::new(r"^.*?(1|2|3|4|5|6|7|8|9|one|two|three|four|five|six|seven|eight|nine)").unwrap();
    let last_regex = Regex::new(r"^.*(1|2|3|4|5|6|7|8|9|one|two|three|four|five|six|seven|eight|nine)").unwrap();

    let sum: u32 = reader.lines()
        .filter_map(|line| line.ok())
        .map(|line| {

            let first_digit = first_regex.captures(&line).unwrap().get(1).unwrap().as_str();
            let last_digit = last_regex.captures(&line).unwrap().get(1).unwrap().as_str();

            let first_digit = match first_digit {
                "one" => 1,
                "two" => 2,
                "three" => 3,
                "four" => 4,
                "five" => 5,
                "six" => 6,
                "seven" => 7,
                "eight" => 8,
                "nine" => 9,
                _ => first_digit.parse::<u32>().unwrap()
            };

            let last_digit = match last_digit {
                "one" => 1,
                "two" => 2,
                "three" => 3,
                "four" => 4,
                "five" => 5,
                "six" => 6,
                "seven" => 7,
                "eight" => 8,
                "nine" => 9,
                _ => last_digit.parse::<u32>().unwrap()
            };

            first_digit * 10 + last_digit
        })
        .sum();

    println!("Sum: {}", sum);

    Ok(())
}