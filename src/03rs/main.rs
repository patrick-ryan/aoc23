use std::collections::{HashMap, HashSet};
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

fn parse_int(s: &str) -> i32 {
    s.parse::<i32>().unwrap()
}

fn parse(path: &Path) -> Vec<Vec<char>> {
    let mut grid: Vec<Vec<char>> = Vec::new();
    if let Ok(lines) = read_lines(path) {
        for line_result in lines {
            if let Ok(line) = line_result {
                if line == "" {
                    continue;
                } else {
                    let row = line.chars().collect_vec();
                    grid.push(row);
                }
            }
        }
    } else {
        panic!();
    }
    return grid;
}

fn get_border_points(
    current_point: &(i32, i32),
    max_x: &i32,
    max_y: &i32,
) -> HashSet<(i32, i32)> {
    let mut result = HashSet::new();
    if current_point.0 > 0 {
        result.insert((current_point.0 - 1, current_point.1));
        if current_point.1 > 0 {
            result.insert((
                current_point.0 - 1,
                current_point.1 - 1,
            ));
        }
        if current_point.1 < *max_y {
            result.insert((
                current_point.0 - 1,
                current_point.1 + 1,
            ));
        }
    }
    if current_point.0 < *max_x {
        result.insert((current_point.0 + 1, current_point.1));
        if current_point.1 > 0 {
            result.insert((
                current_point.0 + 1,
                current_point.1 - 1,
            ));
        }
        if current_point.1 < *max_y {
            result.insert((
                current_point.0 + 1,
                current_point.1 + 1,
            ));
        }
    }
    if current_point.1 > 0 {
        result.insert((current_point.0, current_point.1 - 1));
    }
    if current_point.1 < *max_y {
        result.insert((current_point.0, current_point.1 + 1));
    }
    return result;
}

fn is_part_number(
    grid: &Vec<Vec<char>>,
    point: &(i32, i32),
) -> (bool, bool) {
    let max_x = (grid[0].len() - 1) as i32;
    let max_y = (grid.len() - 1) as i32;

    let cell = grid[point.0 as usize][point.1 as usize];

    return match cell {
        '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8'
        | '9' => {
            let mut has_symbol = false;
            for surrounding_point in
                get_border_points(point, &max_x, &max_y)
            {
                let surr_cell = grid
                    [surrounding_point.0 as usize]
                    [surrounding_point.1 as usize];
                match surr_cell {
                    '0' | '1' | '2' | '3' | '4' | '5' | '6'
                    | '7' | '8' | '9' | '.' => has_symbol = false,
                    _ => has_symbol = true,
                }
                if has_symbol {
                    break;
                }
            }
            (true, has_symbol)
        }
        _ => (false, false),
    };
}

fn get_parts(grid: &Vec<Vec<char>>) -> Vec<String> {
    let max_x = (grid[0].len() - 1) as i32;
    let max_y = (grid.len() - 1) as i32;

    let mut result = Vec::new();

    let mut current_part = String::new();
    let mut current_part_valid = false;
    for point in (0..=max_x).cartesian_product(0..=max_y) {
        let (is_number, has_symbol) =
            is_part_number(grid, &point);
        if is_number {
            current_part
                .push(grid[point.0 as usize][point.1 as usize]);
            if has_symbol {
                current_part_valid = true;
            }
        } else {
            if current_part.len() > 0 {
                if current_part_valid {
                    result.push(current_part);
                }
                current_part = String::new();
                current_part_valid = false;
            }
        }
    }

    return result;
}

fn get_gears(
    grid: &Vec<Vec<char>>,
) -> HashMap<(i32, i32), HashSet<String>> {
    let max_x = (grid[0].len() - 1) as i32;
    let max_y = (grid.len() - 1) as i32;

    let mut gears = HashMap::new();

    for point in (0..=max_x).cartesian_product(0..=max_y) {
        if grid[point.0 as usize][point.1 as usize] == '*' {
            gears.insert(point, HashSet::new());
        }
    }

    return gears;
}

fn update_gear_parts(
    grid: &Vec<Vec<char>>,
    gears: &mut HashMap<(i32, i32), HashSet<String>>,
) {
    // part 2
    let max_x = (grid[0].len() - 1) as i32;
    let max_y = (grid.len() - 1) as i32;

    let mut current_part = String::new();
    let mut current_part_valid = false;
    let mut current_part_gears: HashSet<(i32, i32)> =
        HashSet::new();
    for point in (0..=max_x).cartesian_product(0..=max_y) {
        let (is_number, _) = is_part_number(grid, &point);
        if is_number {
            current_part
                .push(grid[point.0 as usize][point.1 as usize]);
            current_part_valid = true;

            let surrounding_points =
                get_border_points(&point, &max_x, &max_y);
            for surrounding_point in surrounding_points {
                if gears.contains_key(&surrounding_point) {
                    current_part_gears.insert(surrounding_point);
                }
            }
        } else {
            if current_part.len() > 0 {
                if current_part_valid {
                    for gear_point in current_part_gears.iter() {
                        let new_gear_parts: HashSet<String> =
                            gears[gear_point]
                                .union(&HashSet::from([
                                    current_part.clone(),
                                ]))
                                .map(|x| x.to_owned())
                                .collect();
                        gears.insert(*gear_point, new_gear_parts);
                    }
                }
                current_part = String::new();
                current_part_valid = false;
                current_part_gears = HashSet::new();
            }
        }
    }
}

fn main() {
    // let path_buf =
    //     Path::new(file!()).parent().unwrap().join("ex.in.txt");
    let path_buf =
        Path::new(file!()).parent().unwrap().join("in.txt");

    assert!(path_buf.as_path().exists());

    let grid = parse(path_buf.as_path());

    // let parts = get_parts(&grid);

    // let total = parts
    //     .iter()
    //     .fold(0, |sum, y| sum + parse_int(y.as_str()));

    let mut gears = get_gears(&grid);

    update_gear_parts(&grid, &mut gears);

    let valid_gears =
        gears.values().filter(|x| x.len() == 2).collect_vec();
    let total = valid_gears.iter().fold(0, |sum, x| {
        sum + x.iter().fold(1, |s, y| s * parse_int(y.as_str()))
    });

    println!("Total is: {}", total);
}
