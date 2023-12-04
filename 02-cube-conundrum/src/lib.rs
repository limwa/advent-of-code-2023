use regex::Regex;

pub struct Game {
    pub id: u32,
    pub rounds: Vec<Round>
}

impl Game {
    fn new(id: u32) -> Game {
        Game {
            id,
            rounds: Vec::new()
        }
    }
}

#[derive(Default)]
pub struct Round {
    pub red: u32,
    pub green: u32,
    pub blue: u32
}

pub fn read_game_data(reader: impl Iterator<Item = String>) -> Vec<Game> {
    let game_id_regex = Regex::new(r"^Game (\d+)").unwrap();
    let game_info_regex = Regex::new(r"([ ,]{1,2}\d+ (blue|red|green))+").unwrap();
    let cubes_regex = Regex::new(r"(?P<count>\d+) (?P<color>blue|red|green)").unwrap();

    let games = reader
        .map(|line| {
            let game_id = game_id_regex.captures(&line)
                .and_then(|capture| capture.get(1))
                .map(|group| group.as_str())
                .map(|val| val.parse::<u32>())
                .and_then(|val| val.ok())
                .unwrap();

            let game_info = game_info_regex.captures_iter(&line)
                .filter_map(|capture| capture.get(0).map(|group| group.as_str()))
                .map(|val| {

                    cubes_regex.captures_iter(val)
                        .map(|capture| {
                            let number = capture.name("count")
                                .map(|group| group.as_str())
                                .map(|val| val.parse::<u32>())
                                .and_then(|val| val.ok())
                                .unwrap();

                            let color = capture.name("color")
                                .map(|group| group.as_str())
                                .unwrap();

                            (number, color)
                        })
                        .fold(Round::default(), |mut acc, (count, color)| {
                            match color {
                                "red" => acc.red += count,
                                "green" => acc.green += count,
                                "blue" => acc.blue += count,
                                _ => unreachable!()
                            }

                            acc
                        })

                });

            let game = game_info.fold(Game::new(game_id), |mut acc, round| {
                acc.rounds.push(round);
                acc
            });

            game               
        })
        .collect::<Vec<Game>>();

    games
}