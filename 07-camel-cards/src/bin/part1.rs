use std::{fs, io::{self, BufRead}, collections::HashMap};
use camel_cards::*;

const CARD_POWER_ORDER: &str = "AKQJT98765432";

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum PlayKind {
    FiveOfKind,
    FourOfKind,
    FullHouse,
    ThreeOfKind,
    TwoPair,
    OnePair,
    HighCard,
}

fn main() -> io::Result<()> {
    let file = fs::File::open("input.txt")?;
    let lines = io::BufReader::new(file).lines()
        .filter_map(|line| line.ok());

    let mut plays = read_plays(lines);
    plays.sort_by_cached_key(|play| (
        determine_play_kind(play),
        play.cards.iter().map(|&char| CARD_POWER_ORDER.find(char).unwrap()).collect::<Vec<_>>()
    ));

    let sum: i32 = plays.iter()
        .rev()
        .enumerate()
        .map(|(index, play)| (index + 1, play))
        .map(|(rank, play)| play.bid * rank as i32)
        .sum();

    println!("{}", sum);

    Ok(())
}

fn determine_play_kind(play: &Play) -> PlayKind {
    let mut map: HashMap<char, u8> = HashMap::new();

    for &card in &play.cards {
        *map.entry(card).or_insert(0) += 1;
    }

    let mut fives = 0;
    let mut fours = 0;
    let mut threes = 0;
    let mut twos = 0;
    let mut ones = 0;

    for (_, &count) in map.iter() {
        match count {
            1 => ones += 1,
            2 => twos += 1,
            3 => threes += 1,
            4 => fours += 1,
            5 => fives += 1,
            _ => (),
        }
    }

    if fives > 0 {
        return PlayKind::FiveOfKind;
    }

    if fours > 0 {
        return PlayKind::FourOfKind;
    }

    if threes > 0 {
        if twos > 0 {
            return PlayKind::FullHouse;
        }

        return PlayKind::ThreeOfKind;
    }

    if twos > 1 {
        return PlayKind::TwoPair;
    }

    if twos > 0 {
        return PlayKind::OnePair;
    }

    return PlayKind::HighCard;
}