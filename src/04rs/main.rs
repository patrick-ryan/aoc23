use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use regex::Regex;

fn read_lines<P>(
    filename: P,
) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_int(s: &str) -> i32 {
    s.parse::<i32>().unwrap()
}

fn parse(path: &Path) -> i32 {
    let mut total = 0;
    let mut card_counts: HashMap<i32, i32> = HashMap::new();
    if let Ok(lines) = read_lines(path) {
        for line_result in lines {
            if let Ok(line) = line_result {
                let re = Regex::new(
                    r"Card +(?P<cardn>[0-9]+): (?P<winning_numbers>.+) \| (?P<card_numbers>.+)"
                ).unwrap();
                let caps = re.captures(&line).unwrap();

                let cardn = parse_int(
                    caps.name("cardn").unwrap().as_str(),
                );

                let winning_numbers: HashSet<i32> = caps
                    .name("winning_numbers")
                    .unwrap()
                    .as_str()
                    .split_whitespace()
                    .map(parse_int)
                    .collect::<HashSet<i32>>();

                let card_numbers: HashSet<i32> = caps
                    .name("card_numbers")
                    .unwrap()
                    .as_str()
                    .split_whitespace()
                    .map(parse_int)
                    .collect::<HashSet<i32>>();

                let card_winning_numbers: HashSet<&i32> =
                    winning_numbers
                        .intersection(&card_numbers)
                        .collect();

                if !card_counts.contains_key(&cardn) {
                    card_counts.insert(cardn, 1);
                }

                if !card_winning_numbers.is_empty() {
                    // total += 2_i32.pow(
                    //     (card_winning_numbers.len() - 1) as u32,
                    // );

                    // part 2
                    for i in 1..=(card_winning_numbers.len()) {
                        let next_cardn = cardn + (i as i32);
                        if !card_counts.contains_key(&next_cardn)
                        {
                            // start with 1 card
                            card_counts.insert(next_cardn, 1);
                        }
                        // add newly won cards
                        card_counts.insert(
                            next_cardn,
                            card_counts[&next_cardn]
                                + card_counts[&cardn],
                        );
                    }
                }
            }
        }
        total = card_counts.values().sum();
    } else {
        panic!();
    }
    return total;
}

fn main() {
    // let path_buf =
    //     Path::new(file!()).parent().unwrap().join("ex.in.txt");
    let path_buf =
        Path::new(file!()).parent().unwrap().join("in.txt");

    assert!(path_buf.as_path().exists());

    let total = parse(path_buf.as_path());

    println!("Total is: {}", total);
}
