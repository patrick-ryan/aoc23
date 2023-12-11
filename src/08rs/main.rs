use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use itertools::Itertools;
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

fn parse(
    path: &Path,
) -> (Vec<char>, HashMap<String, (String, String)>) {
    let instructions;
    let network;
    if let Ok(mut lines) = read_lines(path) {
        instructions = lines
            .next()
            .unwrap()
            .ok()
            .unwrap()
            .chars()
            .collect_vec();

        let re = Regex::new(
            r"(?P<node>[A-Z0-9]{3}) = \((?P<left>[A-Z0-9]{3}), (?P<right>[A-Z0-9]{3})\)"
        ).unwrap();

        let network_iter = lines.skip(1).map(|line_result| {
            let line = line_result.ok().unwrap();
            let caps = re.captures(&line).unwrap();
            (
                caps.name("node").unwrap().as_str().to_string(),
                (
                    caps.name("left")
                        .unwrap()
                        .as_str()
                        .to_string(),
                    caps.name("right")
                        .unwrap()
                        .as_str()
                        .to_string(),
                ),
            )
        });
        network = HashMap::from_iter(network_iter);
    } else {
        panic!();
    }
    return (instructions, network);
}

fn get_steps_to_end(
    instructions: &Vec<char>,
    network: &HashMap<String, (String, String)>,
) -> i32 {
    // part 1
    let mut current_node: String = "AAA".to_string();
    let mut current_instruction: usize = 0 as usize;
    let mut step_count = 0;
    loop {
        let next_node;
        if instructions[current_instruction as usize] == 'L' {
            next_node = network[&current_node].0.clone();
        } else {
            next_node = network[&current_node].1.clone();
        }

        step_count += 1;
        current_instruction += 1;
        if current_instruction == instructions.len() {
            current_instruction = 0;
        }
        current_node = next_node.clone();

        if current_node == "ZZZ" {
            break;
        }
    }
    return step_count;
}

#[derive(Debug)]
struct NodeTrack {
    start_node: String,
    visited: Vec<(String, String)>,
    end_distances: Vec<i64>,
    frozen_loop: bool,
    step: i64,
}

fn get_steps_to_end_2(
    instructions: &Vec<char>,
    network: &HashMap<String, (String, String)>,
) -> i64 {
    // part 2
    let mut node_tracks: Vec<NodeTrack> = network
        .keys()
        .filter(|x| x.ends_with('A'))
        .map(|x| NodeTrack {
            start_node: x.clone(),
            visited: vec![(
                instructions[0].to_string(),
                x.clone(),
            )],
            end_distances: Vec::new(),
            frozen_loop: false,
            step: 0,
        })
        .collect_vec();

    let mut current_instruction: usize = 0 as usize;
    let mut step_count: i64 = 0;
    loop {
        if node_tracks.iter().all(|x| x.frozen_loop) {
            break;
        }

        let next_instruction;
        if current_instruction + 1 == instructions.len() {
            next_instruction = 0;
        } else {
            next_instruction = current_instruction + 1;
        }
        let current_instruction_char =
            instructions[current_instruction as usize];

        // loop into all tracks have cycled
        for track in
            node_tracks.iter_mut().filter(|nt| !nt.frozen_loop)
        {
            let current_node = track
                .visited
                .last()
                .unwrap_or(&(
                    current_instruction_char.to_string(),
                    track.start_node.clone(),
                ))
                .1
                .clone();

            let next_node;
            if current_instruction_char == 'L' {
                next_node =
                    network.get(&current_node).unwrap().0.clone();
            } else {
                next_node =
                    network.get(&current_node).unwrap().1.clone();
            }

            let next_instruct_node = (
                instructions[..=next_instruction]
                    .into_iter()
                    .collect(),
                next_node.clone(),
            );
            let maybe_visited_item = track
                .visited
                .binary_search_by(|probe| {
                    probe.cmp(&next_instruct_node)
                })
                .ok();
            if let Some(visited_item_index) = maybe_visited_item {
                // track has cycled
                let cycle_len =
                    track.visited.len() - visited_item_index;
                track.frozen_loop = true;

                // end distances are now computed by distance between each other
                let mut new_end_distances = Vec::new();
                let mut last_step = 0;
                let mut first_step = 0;
                for end_distance in &track.end_distances {
                    let dist_from_cycle_start = end_distance
                        - (visited_item_index as i64);
                    if dist_from_cycle_start < 0 {
                        continue;
                    }
                    if first_step == 0 {
                        first_step = dist_from_cycle_start;
                        last_step = dist_from_cycle_start;
                    } else {
                        new_end_distances.push(
                            dist_from_cycle_start - last_step,
                        );
                        last_step = dist_from_cycle_start;
                    }
                }
                new_end_distances.push(
                    (cycle_len as i64) - last_step + first_step,
                );
                track.step = track.end_distances[0];
                track.end_distances = new_end_distances;
            } else {
                // track continues
                track.visited.push(next_instruct_node);

                if next_node.ends_with('Z') {
                    track.end_distances.push(step_count + 1);
                }
            }
        }

        step_count += 1;
        current_instruction = next_instruction;
    }

    // use LCM to compute the first step where all node tracks have reached an end;
    // I cheated and looked at the end_distances, there is only ever one per track,
    // but this could be pretty easily updated to cartesian product the different
    // end distances of all tracks to do LCM on
    return node_tracks
        .iter()
        .map(|x| x.end_distances[0])
        .reduce(|acc, x| num::integer::lcm(acc, x))
        .unwrap();
}

fn main() {
    // let path_buf =
    //     Path::new(file!()).parent().unwrap().join("ex2.in.txt");
    let path_buf =
        Path::new(file!()).parent().unwrap().join("in.txt");

    assert!(path_buf.as_path().exists());

    let (instructions, network) = parse(path_buf.as_path());

    let total = get_steps_to_end_2(&instructions, &network);

    println!("Total is: {}", total);
}

// scratch

// naive approach to part 2, going one step at a time - this would
// definitely take too long, never finished running

// let mut current_nodes: Vec<String> = network
//     .keys()
//     .filter(|x| x.ends_with('A'))
//     .map(|x| x.to_owned())
//     .collect_vec();
// let mut current_instruction: usize = 0 as usize;
// let mut step_count = 0;
// loop {
//     let mut next_nodes: Vec<String> = Vec::new();
//     for node in current_nodes.iter() {
//         let next_node;
//         if instructions[current_instruction as usize] == 'L' {
//             next_node = network.get(node).unwrap().0.clone();
//         } else {
//             next_node = network.get(node).unwrap().1.clone();
//         }
//         next_nodes.push(next_node);
//     }
//     current_nodes = next_nodes.clone();

//     step_count += 1;
//     current_instruction += 1;
//     if current_instruction == instructions.len() {
//         current_instruction = 0;
//     }

//     if current_nodes.iter().all(|x| x.ends_with('Z')) {
//         break;
//     }
// }
// return step_count;

// smart approach to part 2 - this algorithm worked fine actually, but it
// did take a while to compute; basically doing LCM the hard way

// now that all node tracks have been computed, increase the steps until
// we arrive at a single step (with all end nodes)
// loop {
//     if node_tracks.iter().map(|nt| nt.step).all_equal() {
//         break;
//     }

//     // update the minimum-step node track to the next end
//     let min_node_track: &mut NodeTrack = node_tracks
//         .iter_mut()
//         .min_by(|x, y| x.step.cmp(&y.step))
//         .unwrap();

//     min_node_track.step += min_node_track.end_distances
//         [min_node_track.current_end];
//     let mut new_end = min_node_track.current_end + 1;
//     if new_end >= min_node_track.end_distances.len() {
//         new_end = 0;
//     }
//     min_node_track.current_end = new_end;
// }

// return node_tracks.first().unwrap().step;
