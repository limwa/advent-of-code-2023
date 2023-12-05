use std::{fs, io::{self, BufRead}};

use if_you_give_a_seed_a_fertilizer::*;

fn map_range_over_ranges(base_range: &RangeMapping, possible_ranges: &Vec<RangeMapping>) -> Vec<RangeMapping> {
    let mut new_ranges = Vec::<RangeMapping>::new();
    
    let mut current_source_value = base_range.source_start;
    let mut possible_ranges_iter = possible_ranges.iter();

    while current_source_value < base_range.get_source_end() {
        let next_source_value = current_source_value + base_range.get_mapping_offset();

        let next_range = possible_ranges_iter.next();
        if next_range.is_none() {

            let items_left = base_range.get_source_end() - current_source_value;
            
            // If there is no applicable range, we map as-is
            new_ranges.push(RangeMapping {
                source_start: current_source_value,
                destination_start: next_source_value,
                size: items_left,
            });

            current_source_value += items_left;
            continue;
        }

        let next_range = next_range.unwrap();
        let items_left_in_base_range = base_range.get_source_end() - current_source_value;
        
        if next_source_value < next_range.source_start {

            // If there is no applicable range, we map as-is
            let items_until_in_range = next_range.source_start - next_source_value;
            
            new_ranges.push(RangeMapping {
                source_start: current_source_value,
                destination_start: next_source_value,
                size: items_left_in_base_range.min(items_until_in_range),
            });

            current_source_value += items_until_in_range;
            continue;
        }

        if next_source_value >= next_range.get_source_end() {
            continue;
        }

        let items_left_in_next_range = next_range.get_source_end() - next_source_value;
        let mapped_size = items_left_in_base_range.min(items_left_in_next_range);

        new_ranges.push(RangeMapping {
            source_start: current_source_value,
            destination_start: next_source_value + next_range.get_mapping_offset(),
            size: mapped_size,
        });

        current_source_value += mapped_size;
    }
    
    new_ranges
}

fn main() -> io::Result<()> {
    let file = fs::File::open("input.txt")?;
    let lines = io::BufReader::new(file).lines()
        .filter_map(|line| line.ok());

    let mut almanac = read_almanac(lines);

    for category in &mut almanac.mappings {
        category.ranges.sort_by_key(|range| range.source_start);
    }

    let mut mapped_category = CategoryMapping::default();
    mapped_category.source_category = "seed".to_owned();
    mapped_category.destination_category = "seed".to_owned();

    almanac.seeds.iter()
        .zip(almanac.seeds.iter().skip(1))
        .step_by(2)
        .for_each(|(&start, &size)| {
            mapped_category.ranges.push(RangeMapping {
                source_start: start,
                destination_start: start,
                size,
            });
        });

    let mapped_category = almanac.mappings.iter()
        .fold(mapped_category, |acc, category| {
            let new_ranges = acc.ranges.iter()
                .flat_map(|base_range| map_range_over_ranges(base_range, &category.ranges))
                .collect::<Vec<_>>();

            CategoryMapping {
                source_category: acc.source_category,
                destination_category: category.destination_category.to_string(),
                ranges: new_ranges,
            }
        });

    let min_location = mapped_category.ranges.iter()
        .map(|range| range.destination_start)
        .min();

    println!("Value: {:#?}", min_location.unwrap());

    Ok(())
}