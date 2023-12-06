use std::ops::{Range};

use regex::Regex;

pub struct Race {
    pub time: i64,
    pub min_distance: i64,
}

pub fn read_races(reader: impl Iterator<Item = String>) -> Vec<Race> {
    let numbers_regex = Regex::new(r"\d+").unwrap();

    let mut parsed_lines = reader.map(|line| {
        numbers_regex.captures_iter(&line)
            .map(|capture| {
                let number = capture.get(0)
                    .map(|group| group.as_str())
                    .map(|val| val.parse::<i64>())
                    .and_then(|val| val.ok())
                    .unwrap();

                number
            })
            .collect::<Vec<_>>()
    });

    let times = parsed_lines.next().unwrap();
    let min_distances = parsed_lines.next().unwrap();

    times.iter()
        .zip(min_distances.iter())
        .map(|(&time, &min_distance)| Race { time, min_distance })
        .collect::<Vec<_>>()
}


pub fn find_possible_charging_times(race: &Race) -> Range<i64> {
    // the distance moved in a race is d = (T - c) * v, where c is the charging time
    // note that v = c, in this problem

    // d > min_distance and c <= T
    
    // min_distance < (T - c) * v
    // min_distance < T * v - c * v
    // min_distance < - c^2 + T * c
    // c^2 - T * c + min_distance < 0

    // c = [T +- sqrt(T^2 - 4 * min_distance)] / 2

    // c > [T - sqrt(T^2 - 4 * min_distance)] / 2
    // c < [T + sqrt(T^2 - 4 * min_distance)] / 2
    
    let delta = (race.time.pow(2) - 4 * race.min_distance) as f64;
    let delta = delta.sqrt();

    let min_charging_time = (race.time as f64 - delta) / 2.0;
    let min_charging_time = (min_charging_time + 1.0).floor();


    let max_charging_time = (race.time as f64 + delta) / 2.0;
    let max_charging_time = (max_charging_time - 1.0).ceil();

    min_charging_time as i64 .. max_charging_time as i64 + 1
}