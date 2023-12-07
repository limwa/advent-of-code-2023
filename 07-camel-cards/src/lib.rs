#[derive(Debug)]
pub struct Play {
    pub cards: Vec<char>,
    pub bid: i32,
}

pub fn read_plays(reader: impl Iterator<Item = String>) -> Vec<Play> {
    reader.map(|line| {
        let mut split = line.split(" ");

        let cards = split.next().unwrap().chars().collect::<Vec<_>>();
        let bid = split.next().unwrap().parse::<i32>().unwrap();

        Play { bid, cards }
    })
    .collect()
}

