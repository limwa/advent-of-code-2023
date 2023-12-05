use std::{fs, io::{self, BufRead}, collections::HashMap};

use scratchcards::*;

#[derive(Debug)]
struct DecrementingCounters {
    marker: usize,
    active_counters: usize,
    counters: HashMap<usize, usize>,
}

impl DecrementingCounters {
    fn new() -> DecrementingCounters {
        return DecrementingCounters {
            marker: 0,
            active_counters: 0,
            counters: HashMap::new()
        };
    }

    fn active_counters(&self) -> usize {
        self.active_counters
    }

    fn add_counters(&mut self, copies: usize, counter: usize) {
        let end = self.marker + counter;
        let entry = self.counters.entry(end);
        *entry.or_insert(0) += copies;

        self.active_counters += copies;
    }

    fn decrement_counters(&mut self) {
        self.marker += 1;
        if let Some(expired_counters) = self.counters.remove(&self.marker) {
            self.active_counters -= expired_counters;
        }
    }
}

fn main() -> io::Result<()> {
    
    let file = fs::File::open("input.txt")?;
    let lines = io::BufReader::new(file).lines()
        .filter_map(|line| line.ok());

    let scratchcards = read_pile_of_scratchcards(lines);

    let winning_numbers_per_card = scratchcards.iter()
        .map(|scratchcard| {
            scratchcard.played_numbers.iter()
                .filter(|played_number| scratchcard.winning_numbers.contains(played_number))
                .count()
        })
        .collect::<Vec<usize>>();


    let mut total_cards = 0;
    let mut pending_copies = DecrementingCounters::new();

    for winning_numbers_num in winning_numbers_per_card {
        let copies = pending_copies.active_counters() + 1;
        pending_copies.decrement_counters();
        
        total_cards += copies;

        if winning_numbers_num > 0 {
            pending_copies.add_counters(copies, winning_numbers_num);
        }
    }

    println!("Copies: {:?}", total_cards);

    Ok(())
}
