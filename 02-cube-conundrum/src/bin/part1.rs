use std::{fs, io::{self, BufRead}};

use cube_conundrum::{read_game_data, Round};

fn main() -> io::Result<()> {

    let file = fs::File::open("input.txt")?;
    let lines = io::BufReader::new(file).lines()
        .filter_map(|line| line.ok());

    let games = read_game_data(lines);

    let sum: u32 = games.iter()
        .filter_map(|game| {
            let max_round = game.rounds.iter()
                .fold(Round::default(), |mut acc, round| {
                    acc.red = acc.red.max(round.red);
                    acc.green = acc.green.max(round.green);
                    acc.blue = acc.blue.max(round.blue);

                    acc
                });

            if max_round.red <= 12 && max_round.green <= 13 && max_round.blue <= 14 {
                Some(game.id)
            } else {
                None
            }
        })
        .sum();

    println!("Sum: {}", sum);
    
    Ok(())
}