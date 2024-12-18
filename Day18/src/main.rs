use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fs;
use std::io;

const DIRECTIONS: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

#[derive(Debug)]
struct Edge {
    node: (usize, usize),
    weight: u32,
}

fn generate_graph(
    restricted_coords: &HashSet<(usize, usize)>,
    width: usize,
    height: usize,
) -> HashMap<(usize, usize), Vec<Edge>> {
    let mut graph = HashMap::new();
    let rows = height;
    let cols = width;

    for row in 0..rows {
        for col in 0..cols {
            if restricted_coords.contains(&(row, col)) {
                continue;
            }

            let mut neighbors = Vec::new();

            for &(dr, dc) in DIRECTIONS.iter() {
                let new_row = row as isize + dr;
                let new_col = col as isize + dc;
                if new_row >= 0
                    && new_row < rows as isize
                    && new_col >= 0
                    && new_col < cols as isize
                {
                    let new_row = new_row as usize;
                    let new_col = new_col as usize;

                    if !restricted_coords.contains(&(new_row, new_col)) {
                        neighbors.push(Edge {
                            node: (new_row, new_col),
                            weight: 1,
                        });
                    }
                }
            }
            graph.insert((row, col), neighbors);
        }
    }
    graph
}

fn dijkstra(
    graph: &HashMap<(usize, usize), Vec<Edge>>,
    start: (usize, usize),
    end: (usize, usize),
) -> Option<u32> {
    let mut distances: HashMap<(usize, usize), u32> = HashMap::new();
    let mut heap = BinaryHeap::new();
    let mut visited = HashSet::new();

    distances.insert(start, 0);
    heap.push(Reverse((0, start)));

    while let Some(Reverse((current_distance, current_node))) = heap.pop() {
        let (row, col) = current_node;

        if (row, col) == end {
            return Some(current_distance);
        }

        if !visited.insert(current_node) {
            continue;
        }

        if let Some(neighbors) = graph.get(&current_node) {
            for edge in neighbors {
                let next_node = edge.node;
                let next_distance = current_distance + edge.weight;

                if next_distance < *distances.get(&next_node).unwrap_or(&u32::MAX) {
                    distances.insert(next_node, next_distance);
                    heap.push(Reverse((next_distance, next_node)));
                }
            }
        }
    }

    None
}

fn solve_part1() {
    let restricted_coords: HashSet<(usize, usize)> = fs::read_to_string("input.txt")
        .expect("Input.txt file is missing")
        .lines()
        .take(1024)
        .map(|line| {
            let parts: Vec<usize> = line.split(',').map(|x| x.parse().unwrap()).collect();
            (parts[0], parts[1])
        })
        .collect();

    let width = 71;
    let height = 71;
    let start = (0, 0);
    let end = (70, 70);

    let graph = generate_graph(&restricted_coords, width, height);

    if let Some(distance) = dijkstra(&graph, start, end) {
        println!("Shortest path distance: {}", distance);
    } else {
        println!("No path found.");
    }
}

fn graph_connected(restricted_coords: HashSet<(usize, usize)>) -> bool {
    let width = 71;
    let height = 71;
    let start = (0, 0);
    let end = (70, 70);

    let graph = generate_graph(&restricted_coords, width, height);

    dijkstra(&graph, start, end).is_some()
}

fn solve_part2() {
    let restricted_coords: Vec<(usize, usize)> = fs::read_to_string("input.txt")
        .expect("Input.txt file is missing")
        .lines()
        .map(|line| {
            let parts: Vec<usize> = line.split(',').map(|x| x.parse().unwrap()).collect();
            (parts[0], parts[1])
        })
        .collect();

    let mut low = 0;
    let mut high = restricted_coords.len() - 1;

    while low < high {
        let mid = (high + low) / 2;
        let restricted_set = restricted_coords[..mid + 1].iter().cloned().collect();
        if graph_connected(restricted_set) {
            low = mid + 1;
        } else {
            high = mid;
        }
    }

    println!(
        "Coordinate that cuts off the path: {:?}",
        restricted_coords[low]
    );
}

fn main() -> io::Result<()> {
    solve_part1();
    solve_part2();

    Ok(())
}
