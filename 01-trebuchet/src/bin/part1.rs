use regex::Regex;
use std::{fs, io::{self, Write, BufRead}};

fn main() -> io::Result<()> {
    let extract_calibration_value = create_calibration_value_extractor();

    let file = fs::File::open("input.txt")?;
    let lines = io::BufReader::new(file).lines();

    let mut output_file = fs::File::create("output/calibration_values.txt")?;
    
    let sum: i32 = lines
        .filter_map(|line| line.ok())
        .map(|line| {
            let calibration_value = extract_calibration_value(&line).unwrap();
            println!("{} --> {}", line, calibration_value);
            writeln!(output_file, "{} --> {}", line, calibration_value).unwrap();
            return calibration_value;
        })
        .map(|calibration_value| calibration_value.parse::<i32>())
        .filter_map(|calibration_value| calibration_value.ok())
        .sum();

    println!("Sum: {}", sum);
    Ok(())
}

fn create_calibration_value_extractor() -> impl Fn(&str) -> Option<String> {
    let regex = Regex::new(r"^[^\d]*(\d).*?(\d)?[^\d]*$").unwrap();

    return move |haystack: &str| {
        let Some(caps) = regex.captures(haystack) else {
            return None;
        };
    
        let first_digit = caps.get(1).unwrap();
        let second_digit = caps.get(2).unwrap_or(first_digit);
    
        let calibration_value= format!("{}{}", first_digit.as_str(), second_digit.as_str());
        return Some(calibration_value);
    }
}
