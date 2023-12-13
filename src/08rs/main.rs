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

#[derive(Debug)]
struct NodeTrack {
    visited: Vec<(String, String)>,
    end_distances: Vec<i64>,
    frozen_loop: bool,
}

fn get_steps_to_end(
    instructions: &Vec<char>,
    network: &HashMap<String, (String, String)>,
) -> i64 {
    // part 2
    let mut node_tracks: Vec<NodeTrack> = network
        .keys()
        .filter(|x| x.ends_with('A'))
        .map(|x| NodeTrack {
            // store the path of each track, identified by its starting node;
            // includes the full instruction sequence that was taken to get to
            // each node
            visited: vec![(
                instructions[0].to_string(),
                x.clone(),
            )],
            // how many steps until an end state was found
            end_distances: Vec::new(),
            // the track looped around once, we no longer need to keep track
            // of its visited path (since it will cycle)
            frozen_loop: false,
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
            let current_node =
                track.visited.last().unwrap().1.clone();

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
    // I cheated and looked at the end states, there is only ever one per track,
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

    let total = get_steps_to_end(&instructions, &network);

    println!("Total is: {}", total);
}
