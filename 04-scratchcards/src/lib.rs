use regex::Regex;

pub struct Scratchcard {
    pub id: u32,
    pub winning_numbers: Vec<u32>,
    pub played_numbers: Vec<u32>,
}

pub fn read_pile_of_scratchcards(reader: impl Iterator<Item = String>) -> Vec<Scratchcard> {
    let card_id_regex = Regex::new(r"Card +(\d+)").unwrap();
    let card_info_regex = Regex::new(r":([\d ]+)\|([\d ]+)").unwrap();
    let numbers_regex = Regex::new(r"\d+").unwrap();

    reader.map(|line| {
        let card_id = card_id_regex.captures(&line)
            .and_then(|capture| capture.get(1))
            .map(|group| group.as_str())
            .map(|val| val.parse::<u32>())
            .and_then(|val| val.ok())
            .unwrap();

        let card_info = card_info_regex.captures(&line).unwrap();
        
        let winning_numbers_str = card_info.get(1)
            .map(|group| group.as_str())
            .unwrap();

        let winning_numbers: Vec<u32> = numbers_regex.captures_iter(&winning_numbers_str)
            .map(|capture| {
                let number = capture.get(0)
                    .map(|group| group.as_str())
                    .map(|val| val.parse::<u32>())
                    .and_then(|val| val.ok())
                    .unwrap();

                number
            })
            .collect();

        let played_numbers_str = card_info.get(2)
            .map(|group| group.as_str())
            .unwrap();

        let played_numbers: Vec<u32> = numbers_regex.captures_iter(&played_numbers_str)
            .map(|capture| {
                let number = capture.get(0)
                    .map(|group| group.as_str())
                    .map(|val| val.parse::<u32>())
                    .and_then(|val| val.ok())
                    .unwrap();

                number
            })
            .collect();

        Scratchcard {
            id: card_id,
            winning_numbers,
            played_numbers,
        }          
    })
    .collect()
}