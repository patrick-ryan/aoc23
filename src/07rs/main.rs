use std::collections::HashMap;
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

fn parse(path: &Path) -> Vec<(String, i32)> {
    let card_map: HashMap<char, char> = HashMap::from([
        ('J', '_'),
        ('2', 'a'),
        ('3', 'b'),
        ('4', 'c'),
        ('5', 'd'),
        ('6', 'e'),
        ('7', 'f'),
        ('8', 'g'),
        ('9', 'h'),
        ('T', 'i'),
        // ('J', 'j'),
        ('Q', 'k'),
        ('K', 'l'),
        ('A', 'm'),
    ]);

    let mut hands: Vec<(String, i32)> = Vec::new();
    if let Ok(lines) = read_lines(path) {
        for line_result in lines {
            if let Ok(line) = line_result {
                let (hand, bid) =
                    line.split(' ').collect_tuple().unwrap();

                let mut normalized_hand = hand.to_string();
                for (letter, mapped) in card_map.iter() {
                    let new_hand = &normalized_hand.replace(
                        letter.to_string().as_str(),
                        mapped.to_string().as_str(),
                    );
                    normalized_hand = new_hand.clone();
                }

                hands.push((normalized_hand, parse_int(bid)));
            }
        }
    } else {
        panic!();
    }
    return hands;
}

fn get_hand_score(cards: &String) -> i32 {
    let counts = cards.chars().counts();
    if counts.len() == 1 {
        // 5 of a kind
        return 6;
    } else if counts.len() == 2 {
        if counts.values().any(|&x| x == (4 as usize)) {
            // 4 of a kind
            return 5;
        } else {
            // full house
            return 4;
        }
    } else if counts.len() == 3 {
        if counts.values().any(|&x| x == (3 as usize)) {
            // 3 of a kind
            return 3;
        } else {
            // two pairs
            return 2;
        }
    } else if counts.len() == 4 {
        // one pair
        return 1;
    } else {
        // high card
        return 0;
    }
}

fn get_hand_score_2(cards: &String) -> i32 {
    // part 2
    let counts = cards.chars().counts();
    let joker_count = counts.get(&'_').unwrap_or(&0).to_owned();
    if counts.len() == 1 || (counts.len() == 2 && joker_count > 0)
    {
        // 5 of a kind
        return 6;
    } else if counts.len() == 2
        || (counts.len() == 3 && joker_count > 0)
    {
        if counts
            .values()
            .any(|&x| x >= (4 - joker_count as usize))
        {
            // 4 of a kind
            return 5;
        } else {
            // full house
            return 4;
        }
    } else if counts.len() == 3
        || (counts.len() == 4 && joker_count > 0)
    {
        if counts
            .values()
            .any(|&x| x >= (3 - joker_count as usize))
        {
            // 3 of a kind
            return 3;
        } else {
            // two pairs
            return 2;
        }
    } else if counts.len() == 4
        || (counts.len() == 5 && joker_count > 0)
    {
        // one pair
        return 1;
    } else {
        // high card
        return 0;
    }
}

fn get_hands_ranked(
    hands: Vec<(String, i32)>,
) -> Vec<(String, i32, i32)> {
    let mut hands_ranked: Vec<(String, i32)> = hands.clone();
    hands_ranked.sort_by(|x, y| {
        let x_hand_score = get_hand_score_2(&x.0);
        let y_hand_score = get_hand_score_2(&y.0);

        if x_hand_score == y_hand_score {
            return x.0.cmp(&y.0);
        } else {
            return x_hand_score.cmp(&y_hand_score);
        }
    });
    return hands_ranked
        .iter()
        .enumerate()
        .map(|x| (x.1 .0.clone(), x.1 .1, (x.0 as i32 + 1)))
        .collect_vec();
}

fn main() {
    // let path_buf =
    //     Path::new(file!()).parent().unwrap().join("ex.in.txt");
    let path_buf =
        Path::new(file!()).parent().unwrap().join("in.txt");

    assert!(path_buf.as_path().exists());

    let hands = parse(path_buf.as_path());
    let hands_ranked = get_hands_ranked(hands);

    let total = hands_ranked
        .iter()
        .fold(0, |sum, (_, bid, rank)| sum + bid * rank);

    println!("Total is: {}", total);
}
