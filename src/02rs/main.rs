use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::HashMap;
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

fn parse(path: &Path) -> HashMap<i32, Vec<Vec<(i32, String)>>> {
    let mut games: HashMap<i32, Vec<Vec<(i32, String)>>> =
        HashMap::new();
    if let Ok(lines) = read_lines(path) {
        for line in lines {
            if let Ok(line) = line {
                let re = Regex::new(
                    r"Game (?P<id>[0-9]+): (?P<reveals>[0-9a-z ,;]+)"
                ).unwrap();
                let caps_iter = re.captures_iter(&line);
                for caps in caps_iter {
                    let game_id = caps
                        .name("id")
                        .unwrap()
                        .as_str()
                        .parse::<i32>()
                        .unwrap();
                    let game_lines =
                        caps.name("reveals").unwrap().as_str();
                    let game_subsets = game_lines
                        .split("; ")
                        .map(|x| {
                            x.split(", ")
                                .map(|y| {
                                    let (a, b) = y
                                        .split(" ")
                                        .map(|x| x.to_owned())
                                        .collect_tuple()
                                        .unwrap();
                                    (a.parse::<i32>().unwrap(), b)
                                })
                                .collect_vec()
                        })
                        .collect_vec();
                    games.insert(game_id, game_subsets);
                }
            }
        }
    } else {
        panic!();
    }
    return games;
}

fn is_game_possible(
    game_config: &HashMap<&str, i32>,
    game_subsets: &Vec<Vec<(i32, String)>>,
) -> bool {
    let mut possible = true;

    for subset in game_subsets {
        for (n, color) in subset {
            if n > &game_config[color.as_str()] {
                possible = false;
                break;
            }
        }
        if !possible {
            break;
        }
    }

    return possible;
}

fn get_game_power(
    game_config: &HashMap<&str, i32>,
    game_subsets: &Vec<Vec<(i32, String)>>,
) -> i32 {
    // part 2
    let mut game_power = 1;
    for (color, _) in game_config {
        let color_power = game_subsets
            .iter()
            .map(|subset| subset.iter().find(|(_, c)| c == color))
            .max_by(|maybe_color_n_a, maybe_color_n_b| {
                match (maybe_color_n_a, maybe_color_n_b) {
                    (Some(color_n_a), Some(color_n_b)) => {
                        color_n_a.cmp(color_n_b)
                    }
                    (Some(_), None) => Ordering::Greater,
                    (None, Some(_)) => Ordering::Less,
                    (None, None) => Ordering::Equal,
                }
            })
            .unwrap();
        game_power *= color_power.unwrap().0;
    }

    return game_power;
}

fn main() {
    // let path_buf =
    //     Path::new(file!()).parent().unwrap().join("ex.in.txt");
    let path_buf =
        Path::new(file!()).parent().unwrap().join("in.txt");

    assert!(path_buf.as_path().exists());

    let game_config: HashMap<&str, i32> =
        [("red", 12), ("green", 13), ("blue", 14)].into();
    let game_results = parse(path_buf.as_path());

    let mut total = 0;
    for (game_id, game_result) in game_results.iter() {
        // if is_game_possible(&game_config, game_result) {
        //     total += game_id;
        // }
        total += get_game_power(&game_config, game_result);
    }

    println!("Total is: {}", total);
}
