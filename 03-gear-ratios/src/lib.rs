use regex::Regex;
use std::ops::Range;

pub struct PartNumber {
    pub cols: Range<i32>,
    pub line: i32,
    pub value: u32,
}

pub struct Symbol {
    pub col: i32,
    pub line: i32,
    pub character: String,
}

#[derive(Default)]
pub struct Schematic {
    pub part_numbers: Vec<PartNumber>,
    pub symbols: Vec<Symbol>,
}

pub fn read_schematic(reader: impl Iterator<Item = String>) -> Schematic {
    let part_number_regex = Regex::new(r"\d+").unwrap();
    let symbol_regex = Regex::new(r"[^\d.\s]").unwrap();

    reader.enumerate()
        .fold(Schematic::default(), |mut schematic, (line_number, content)| {

            part_number_regex.captures_iter(&content)
                .filter_map(|capture| capture.get(0))
                .map(|group| PartNumber {
                    cols: group.start() as i32..group.end() as i32,
                    line: line_number as i32,
                    value: group.as_str().parse().unwrap(),
                })
                .for_each(|part_number| schematic.part_numbers.push(part_number));

            symbol_regex.captures_iter(&content)
                .filter_map(|capture| capture.get(0))
                .map(|group| Symbol {
                    col: group.start() as i32,
                    line: line_number as i32,
                    character: group.as_str().to_owned(),
                })
                .for_each(|symbol| schematic.symbols.push(symbol));

            schematic
        })
}