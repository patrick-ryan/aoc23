use std::fs::File;
use std::io::{self, BufRead};
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

fn parse(path: &Path) -> Vec<Vec<i64>> {
    let mut histories: Vec<Vec<i64>> = Vec::new();
    if let Ok(lines) = read_lines(path) {
        for line_result in lines {
            if let Ok(line) = line_result {
                histories.push(
                    line.split(' ').map(parse_int).collect_vec(),
                );
            }
        }
    } else {
        panic!();
    }
    return histories;
}

fn get_next_sequence_value(
    history: &Vec<i64>,
    prev: bool,
) -> i64 {
    let mut sequences: Vec<Vec<i64>> = vec![history.clone()];
    let mut current_sequence: Vec<i64> = history.clone();
    loop {
        if current_sequence.iter().all_equal() {
            break;
        }
        let next_sequence = current_sequence
            .iter()
            .tuple_windows()
            .map(|(x, y)| y - x)
            .collect_vec();

        sequences.push(next_sequence.clone());

        current_sequence = next_sequence;
    }

    if prev {
        return sequences
            .iter()
            .rev()
            .map(|s| s.first().unwrap())
            .fold(0, |acc, x| x - acc);
    } else {
        return sequences
            .iter()
            .rev()
            .map(|s| s.last().unwrap())
            .sum();
    }
}

fn main() {
    // let path_buf =
    //     Path::new(file!()).parent().unwrap().join("ex.in.txt");
    let path_buf =
        Path::new(file!()).parent().unwrap().join("in.txt");

    assert!(path_buf.as_path().exists());

    let histories = parse(path_buf.as_path());

    let get_next_sequence_value_for_part = |h| {
        return get_next_sequence_value(h, true);
    };

    let total: i64 = histories
        .iter()
        .map(get_next_sequence_value_for_part)
        .sum();

    println!("Total is: {}", total);
}
