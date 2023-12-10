use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::ops::Range;
use std::path::Path;

use itertools::Itertools;

fn read_lines<P>(
    filename: P,
) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_int(s: &str) -> i64 {
    s.parse::<i64>().unwrap()
}

fn parse(
    path: &Path,
) -> (
    Vec<Range<i64>>,
    Vec<HashMap<std::ops::Range<i64>, std::ops::Range<i64>>>,
) {
    // let mut seeds: Vec<i64> = Vec::new();
    let mut seeds: Vec<Range<i64>> = Vec::new();
    let mut seed_to_soil: HashMap<Range<i64>, Range<i64>> =
        HashMap::new();
    let mut soil_to_fertilizer: HashMap<Range<i64>, Range<i64>> =
        HashMap::new();
    let mut fertilizer_to_water: HashMap<Range<i64>, Range<i64>> =
        HashMap::new();
    let mut water_to_light: HashMap<Range<i64>, Range<i64>> =
        HashMap::new();
    let mut light_to_temperature: HashMap<
        Range<i64>,
        Range<i64>,
    > = HashMap::new();
    let mut temperature_to_humidity: HashMap<
        Range<i64>,
        Range<i64>,
    > = HashMap::new();
    let mut humidity_to_location: HashMap<
        Range<i64>,
        Range<i64>,
    > = HashMap::new();
    if let Ok(lines) = read_lines(path) {
        let mut current_map = String::new();
        for line_result in lines {
            if let Ok(line) = line_result {
                if line == "" {
                    continue;
                } else if line.starts_with("seeds") {
                    let seed_numbers = line
                        .split("seeds: ")
                        .last()
                        .unwrap()
                        .split(' ')
                        .map(parse_int);
                    // seeds.extend(seed_numbers)
                    for mut chunk in &seed_numbers.chunks(2) {
                        let start = &chunk.next().unwrap();
                        let range_len = &chunk.last().unwrap();
                        let range = *start..(start + range_len);
                        // let range_vec: Vec<i64> = range.collect();
                        // seeds.extend(range_vec);  // OOM
                        seeds.push(range);
                    }
                } else if line.ends_with(" map:") {
                    current_map = line;
                } else {
                    let (
                        destination_range_start,
                        source_range_start,
                        range_length,
                    ) = line
                        .split(' ')
                        .map(parse_int)
                        .take(3)
                        .next_tuple()
                        .unwrap();
                    let sr = source_range_start
                        ..source_range_start + range_length;
                    let dr = destination_range_start
                        ..destination_range_start + range_length;
                    match current_map.as_str() {
                        "seed-to-soil map:" => {
                            seed_to_soil.insert(sr, dr);
                        }
                        "soil-to-fertilizer map:" => {
                            soil_to_fertilizer.insert(sr, dr);
                        }
                        "fertilizer-to-water map:" => {
                            fertilizer_to_water.insert(sr, dr);
                        }
                        "water-to-light map:" => {
                            water_to_light.insert(sr, dr);
                        }
                        "light-to-temperature map:" => {
                            light_to_temperature.insert(sr, dr);
                        }
                        "temperature-to-humidity map:" => {
                            temperature_to_humidity
                                .insert(sr, dr);
                        }
                        "humidity-to-location map:" => {
                            humidity_to_location.insert(sr, dr);
                        }
                        _ => panic!(),
                    }
                }
            }
        }
    } else {
        panic!();
    }
    return (
        seeds,
        vec![
            seed_to_soil,
            soil_to_fertilizer,
            fertilizer_to_water,
            water_to_light,
            light_to_temperature,
            temperature_to_humidity,
            humidity_to_location,
        ],
    );
}

fn get_seed_location(
    seed: &i64,
    maps: &Vec<
        HashMap<std::ops::Range<i64>, std::ops::Range<i64>>,
    >,
) -> i64 {
    let mut current_value = *seed;
    for garden_map in maps {
        for (source_range, destination_range) in garden_map.iter()
        {
            if source_range.contains(&current_value) {
                let dist = current_value - source_range.start;
                current_value = destination_range.start + dist;
                break;
            }
        }
    }
    return current_value;
}

fn get_overlapping_range(
    r1: &Range<i64>,
    r2: &Range<i64>,
) -> Option<Range<i64>> {
    let mut start = None;
    let mut end = None;
    if r1.start == r2.start {
        start = Some(r1.start);
    } else if r1.start < r2.start {
        if (r1.end - 1) >= r2.start {
            start = Some(r2.start);
        }
    } else {
        // r2.start < r1.start
        if (r2.end - 1) >= r1.start {
            start = Some(r1.start);
        }
    }
    if let Some(_) = start {
        let m = r1.end.min(r2.end);
        end = Some(m);
    }
    if let (Some(s), Some(e)) = (start, end) {
        return Some(s..e);
    } else {
        return None;
    }
}

fn get_seed_range_location(
    seed: &Range<i64>,
    maps: &Vec<
        HashMap<std::ops::Range<i64>, std::ops::Range<i64>>,
    >,
) -> i64 {
    // part 2
    let mut current_value: Vec<Range<i64>> = vec![seed.clone()];
    for garden_map in maps {
        // println!("{current_value:?}");
        let mut new_current_val: Vec<Range<i64>> = Vec::new();
        for current_value_range in current_value {
            let mut non_overlappings: Vec<Range<i64>> =
                vec![current_value_range];
            for (source_range, destination_range) in
                garden_map.iter()
            {
                let mut new_non_overlappings: Vec<Range<i64>> =
                    Vec::new();
                for available_range in non_overlappings {
                    let overlapping = get_overlapping_range(
                        &available_range,
                        source_range,
                    );
                    match overlapping {
                        Some(r) => {
                            let dest_start = destination_range
                                .start
                                + (r.start - source_range.start);
                            let dest_end = destination_range.end
                                - (source_range.end - r.end);
                            new_current_val
                                .push(dest_start..dest_end);
                            if available_range.start < r.start {
                                new_non_overlappings.push(
                                    available_range.start
                                        ..r.start,
                                );
                            }
                            if available_range.end > r.end {
                                new_non_overlappings.push(
                                    r.end..available_range.end,
                                );
                            }
                        }
                        None => {
                            new_non_overlappings.push(
                                available_range.start
                                    ..available_range.end,
                            );
                        }
                    }
                }
                non_overlappings = new_non_overlappings;
            }
            if !non_overlappings.is_empty() {
                new_current_val.extend(non_overlappings);
            }
        }
        current_value = new_current_val;
    }
    return current_value
        .iter()
        .min_by(|x, y| x.start.cmp(&y.start))
        .unwrap()
        .start;
}

fn main() {
    // let path_buf =
    //     Path::new(file!()).parent().unwrap().join("ex.in.txt");
    let path_buf =
        Path::new(file!()).parent().unwrap().join("in.txt");

    assert!(path_buf.as_path().exists());

    let (seeds, maps) = parse(path_buf.as_path());

    // let seed_locations = seeds.iter().map(|seed| {
    //     seed.clone()
    //         .map(|s| get_seed_location(&s, &maps))
    //         .min()
    //         .unwrap()
    // });

    let seed_locations = seeds
        .iter()
        .map(|seed| get_seed_range_location(seed, &maps));

    let total = seed_locations.min().unwrap();

    println!("Total is: {}", total);
}
