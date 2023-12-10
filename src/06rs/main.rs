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

fn parse(path: &Path) -> Vec<(i64, i64)> {
    let races;
    if let Ok(mut lines) = read_lines(path) {
        let times_line = lines.next().unwrap().ok().unwrap();
        let times =
            times_line.split_whitespace().skip(1).map(parse_int);
        let distances_line = lines.next().unwrap().ok().unwrap();
        let distances = distances_line
            .split_whitespace()
            .skip(1)
            .map(parse_int);
        races = distances.zip(times).collect_vec();
    } else {
        panic!();
    }
    return races;
}

fn parse2(path: &Path) -> Vec<(i64, i64)> {
    let races;
    if let Ok(mut lines) = read_lines(path) {
        let time_line = lines.next().unwrap().ok().unwrap();
        let time: String =
            time_line.split_whitespace().skip(1).collect();
        let distance_line = lines.next().unwrap().ok().unwrap();
        let distance: String =
            distance_line.split_whitespace().skip(1).collect();
        races = vec![(
            parse_int(distance.as_str()),
            parse_int(time.as_str()),
        )];
    } else {
        panic!();
    }
    return races;
}

fn get_winning_ways(distance: i64, time: i64) -> i64 {
    let is_odd_time = time % 2 != 0;

    let mut current_speed;
    let mut current_distance_left;

    if is_odd_time {
        current_speed = (time as f64 / 2_f64).floor() as i64;
        current_distance_left =
            (time as f64 / 2_f64).ceil() as i64;
    } else {
        current_speed = time / 2;
        current_distance_left = time / 2;
    }

    let mut ways = 0;
    loop {
        if current_speed * current_distance_left > distance {
            ways += 1;

            current_speed -= 1;
            current_distance_left += 1;

            if current_speed == 0 {
                break;
            }
        } else {
            break;
        }
    }

    if is_odd_time {
        return ways * 2;
    } else {
        return ways * 2 - 1;
    }
}

fn main() {
    // let path_buf =
    //     Path::new(file!()).parent().unwrap().join("ex.in.txt");
    let path_buf =
        Path::new(file!()).parent().unwrap().join("in.txt");

    assert!(path_buf.as_path().exists());

    // let races = parse(path_buf.as_path());
    let races = parse2(path_buf.as_path());

    let mut total = 1;
    for (distance, time) in races {
        total *= get_winning_ways(distance, time);
    }

    println!("Total is: {}", total);
}
