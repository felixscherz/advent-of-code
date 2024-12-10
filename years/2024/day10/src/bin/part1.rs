use std::{
    collections::{HashMap, HashSet},
    io::{stdin, Read},
};

fn apply(
    pos: (usize, usize),
    step: (i32, i32),
    height: usize,
    width: usize,
) -> Option<(usize, usize)> {
    if pos.0 == 0 && step.0 < 0 {
        return None;
    }

    if pos.1 == 0 && step.1 < 0 {
        return None;
    }

    let new_pos = (
        (pos.0 as i32 + step.0) as usize,
        (pos.1 as i32 + step.1) as usize,
    );

    if new_pos.0 >= height || new_pos.1 >= width {
        return None;
    }

    Some(new_pos)
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let mut map: Vec<Vec<u32>> = vec![];
    let mut graph: HashMap<(usize, usize), Vec<(usize, usize)>> = HashMap::new();

    for (_i, line) in input.lines().enumerate() {
        let mut row: Vec<u32> = vec![];
        for (_j, c) in line.chars().enumerate() {
            row.push(c.to_digit(10).unwrap());
        }
        map.push(row);
    }

    let height: usize = map.len();
    let width: usize = map[0].len();

    // construct the graph
    let valid_directions: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
    for (i, row) in map.iter().enumerate() {
        for (j, score) in row.iter().enumerate() {
            for direction in valid_directions {
                match apply((i, j), direction, height, width) {
                    None => continue,
                    Some(other) => {
                        if !(map[other.0][other.1] == score + 1) {
                            continue;
                        };
                        graph.entry((i, j)).or_insert_with(|| vec![]).push(other);
                    }
                };
            }
        }
    }

    let mut sum = 0;

    for start in graph.keys().filter(|x| map[x.0][x.1] == 0) {
        let mut count = 0;
        let mut current_nodes = vec![*start];
        let mut peaks: HashSet<(usize,usize)> = HashSet::new();
        loop {
            // keep a set of current nodes
            // take a step for every node and add those back to the current nodes

            // if a node has score 9, increment the count and don't add ot to the nodes
            let mut new_nodes = vec![];
            for node in &current_nodes {
                match graph.get(&node) {
                    None => continue,
                    Some(nodes) => {
                        for n in nodes {
                            if map[n.0][n.1] == 9 {
                                peaks.insert((n.0, n.1));
                            } else {
                                new_nodes.push(n.clone());
                            }
                        }
                    }
                }
            }
            if new_nodes.is_empty() {
                break;
            }
            current_nodes = new_nodes;
        }
        // eprintln!("start at {},{} has count {}", start.0, start.1, count);
        sum = sum + peaks.len();
    }
    println!("{}", sum);
}
