pub struct Readings {
    pub histories: Vec<ReadingHistory>,
}

pub struct ReadingHistory {
    pub history: Vec<i64>,
}

pub fn read_readings(reader: impl Iterator<Item = String>) -> Readings {
    let mut histories = Vec::new();

    for line in reader {
        let mut history = Vec::new();

        for reading in line.split(' ') {
            history.push(reading.parse::<i64>().unwrap());
        }

        histories.push(ReadingHistory { history });
    }

    Readings { histories }
}
