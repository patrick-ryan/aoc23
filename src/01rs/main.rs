use itertools::Itertools;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(
    filename: P,
) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_calib(line: String) -> u32 {
    let first: u32 = line
        .chars()
        .find_map(|c| c.to_string().parse().ok())
        .unwrap();
    let last: u32 = line
        .chars()
        .rev()
        .find_map(|c| c.to_string().parse().ok())
        .unwrap();

    let calib: String = [
        first.to_string().chars().next().unwrap(),
        last.to_string().chars().next().unwrap(),
    ]
    .into_iter()
    .collect();

    return calib.parse::<u32>().unwrap();
}

fn parse(path: &Path) -> u32 {
    let mut total = 0;
    if let Ok(lines) = read_lines(path) {
        for line in lines {
            if let Ok(line) = line {
                total += get_calib(line);
            }
        }
    } else {
        panic!();
    }
    return total;
}

fn parse2(path: &Path) -> u32 {
    let numbers: HashMap<&str, &str> = HashMap::from([
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ]);

    let mut total = 0;
    if let Ok(lines) = read_lines(path) {
        for line in lines {
            if let Ok(mut line) = line {
                for _ in 0..2 {
                    for (s, number) in
                        numbers.iter().sorted_by_key(|x| x.1)
                    {
                        // slightly cheating... just hoping that prepending the first
                        // char and appending the last char will suffice for the overlapping
                        // numbers (and a second iteration to capture these)
                        let mut new_number =
                            s.chars().next().unwrap().to_string();
                        new_number.push_str(number);
                        new_number
                            .push(s.chars().last().unwrap());
                        line = line.replace(s, &new_number);
                    }
                }
                total += get_calib(line);
            }
        }
    } else {
        panic!();
    }
    return total;
}

fn main() {
    // let path_buf =
    //     Path::new(file!()).parent().unwrap().join("ex.in.txt");
    // let path_buf =
    //     Path::new(file!()).parent().unwrap().join("in.txt");

    // let path_buf =
    //     Path::new(file!()).parent().unwrap().join("ex2.in.txt");
    let path_buf =
        Path::new(file!()).parent().unwrap().join("2.in.txt");

    assert!(path_buf.as_path().exists());

    // let total = parse(path_buf.as_path());
    let total = parse2(path_buf.as_path());

    println!("Total is: {}", total);
}
