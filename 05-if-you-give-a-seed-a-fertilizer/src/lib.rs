use regex::Regex;

#[derive(Default, Debug)]
pub struct Almanac {
    pub seeds: Vec<i64>,
    pub mappings: Vec<CategoryMapping>
}

#[derive(Default, Debug)]
pub struct CategoryMapping {
    pub source_category: String,
    pub destination_category: String,
    pub ranges: Vec<RangeMapping>,
}

#[derive(Debug, Clone)]
pub struct RangeMapping {
    pub source_start: i64,
    pub destination_start: i64,
    pub size: i64,
}

impl RangeMapping {
    pub fn get_source_end(&self) -> i64 {
        return self.source_start + self.size;
    }

    pub fn get_destination_end(&self) -> i64 {
        return self.destination_start + self.size;
    }

    pub fn get_mapping_offset(&self) -> i64 {
        return self.destination_start - self.source_start;
    }
}

pub fn read_almanac(mut lines: impl Iterator<Item = String>) -> Almanac {
    let mut almanac = Almanac::default();

    let first_line = lines.next().unwrap();
    let parts = first_line.split_whitespace();
    parts.skip(1) // first part is "seeds:"
        .map(|s| s.parse::<i64>().unwrap())
        .for_each(|seed| almanac.seeds.push(seed));

    let category_regex = Regex::new(r"(\w+)-to-(\w+) map").unwrap();
    let mut current_category_mapping = CategoryMapping::default();

    for line in lines {
        if line.starts_with(|c: char| c.is_digit(10)) {
            let mut parts = line.split_whitespace();

            let destination_start = parts.next()
                .map(|p| p.parse::<i64>())
                .and_then(|val| val.ok())
                .unwrap();

            let source_start = parts.next()
                .map(|p| p.parse::<i64>())
                .and_then(|val| val.ok())
                .unwrap();

            let size = parts.next()
                .map(|p| p.parse::<i64>())
                .and_then(|val| val.ok())
                .unwrap();

            let range_mapping = RangeMapping {
                source_start,
                destination_start,
                size: size,
            };

            current_category_mapping.ranges.push(range_mapping);
            continue;
        }

        if let Some(captures) = category_regex.captures(&line) {
            almanac.mappings.push(current_category_mapping);

            current_category_mapping = CategoryMapping::default();
            current_category_mapping.source_category = captures[1].to_string();
            current_category_mapping.destination_category = captures[2].to_string();

            continue;
        }
    }

    almanac.mappings.push(current_category_mapping);
    almanac.mappings.remove(0);

    almanac
}