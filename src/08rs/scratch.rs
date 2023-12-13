
// part 1

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


// naive approach to part 2, going one step at a time - this would
// definitely take too long, never finished running

let mut current_nodes: Vec<String> = network
    .keys()
    .filter(|x| x.ends_with('A'))
    .map(|x| x.to_owned())
    .collect_vec();
let mut current_instruction: usize = 0 as usize;
let mut step_count = 0;
loop {
    let mut next_nodes: Vec<String> = Vec::new();
    for node in current_nodes.iter() {
        let next_node;
        if instructions[current_instruction as usize] == 'L' {
            next_node = network.get(node).unwrap().0.clone();
        } else {
            next_node = network.get(node).unwrap().1.clone();
        }
        next_nodes.push(next_node);
    }
    current_nodes = next_nodes.clone();

    step_count += 1;
    current_instruction += 1;
    if current_instruction == instructions.len() {
        current_instruction = 0;
    }

    if current_nodes.iter().all(|x| x.ends_with('Z')) {
        break;
    }
}
return step_count;

// smart approach to part 2 - this algorithm worked fine actually, but it
// did take a while to compute; basically doing LCM the hard way

now that all node tracks have been computed, increase the steps until
we arrive at a single step (with all end nodes)
loop {
    if node_tracks.iter().map(|nt| nt.step).all_equal() {
        break;
    }

    // update the minimum-step node track to the next end
    let min_node_track: &mut NodeTrack = node_tracks
        .iter_mut()
        .min_by(|x, y| x.step.cmp(&y.step))
        .unwrap();

    min_node_track.step += min_node_track.end_distances
        [min_node_track.current_end];
    let mut new_end = min_node_track.current_end + 1;
    if new_end >= min_node_track.end_distances.len() {
        new_end = 0;
    }
    min_node_track.current_end = new_end;
}

return node_tracks.first().unwrap().step;
