use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fs;
use std::io;

// Directions (North, East, South, West)
const DIRECTIONS: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

#[derive(Debug)]
struct Edge {
    node: (usize, usize, usize), // (row, col, direction)
    weight: u32,
}

fn dijkstra(
    graph: &HashMap<(usize, usize, usize), Vec<Edge>>,
    start: (usize, usize, usize),
    end: (usize, usize),
) -> (
    Option<u32>,
    HashMap<(usize, usize, usize), Vec<(usize, usize, usize)>>,
) {
    let mut distances: HashMap<(usize, usize, usize), u32> = HashMap::new();
    let mut heap = BinaryHeap::new();
    let mut visited = HashSet::new();
    let mut predecessors: HashMap<(usize, usize, usize), Vec<(usize, usize, usize)>> =
        HashMap::new();

    distances.insert(start, 0);
    heap.push(Reverse((0, start)));

    while let Some(Reverse((current_distance, current_node))) = heap.pop() {
        let (row, col, _) = current_node;

        if (row, col) == end {
            return (Some(current_distance), predecessors);
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
                    predecessors.insert(next_node, vec![current_node]);
                    heap.push(Reverse((next_distance, next_node)));
                } else if next_distance == *distances.get(&next_node).unwrap_or(&u32::MAX) {
                    predecessors
                        .entry(next_node)
                        .or_default()
                        .push(current_node);
                }
            }
        }
    }

    (None, predecessors)
}

fn find_best_path_tiles(
    predecessors: &HashMap<(usize, usize, usize), Vec<(usize, usize, usize)>>,
    end: (usize, usize),
) -> HashSet<(usize, usize)> {
    let mut tiles_in_paths = HashSet::new();
    let mut stack = Vec::new();

    for dir_index in 0..4 {
        let end_node = (end.0, end.1, dir_index);
        stack.push(end_node);
    }

    while let Some(node) = stack.pop() {
        let (row, col, _) = node;
        tiles_in_paths.insert((row, col));

        if let Some(parents) = predecessors.get(&node) {
            for &parent in parents {
                stack.push(parent);
            }
        }
    }

    tiles_in_paths
}

fn convert_maze_to_graph(maze: &Vec<Vec<char>>) -> HashMap<(usize, usize, usize), Vec<Edge>> {
    let mut graph = HashMap::new();
    let rows = maze.len();
    let cols = maze[0].len();

    for row in 0..rows {
        for col in 0..cols {
            if maze[row][col] == '#' {
                continue;
            }

            for (dir_index, &(dr, dc)) in DIRECTIONS.iter().enumerate() {
                let current_state = (row, col, dir_index);
                let mut neighbors = Vec::new();

                let new_row = row as isize + dr;
                let new_col = col as isize + dc;

                if new_row >= 0
                    && new_row < rows as isize
                    && new_col >= 0
                    && new_col < cols as isize
                {
                    let new_row = new_row as usize;
                    let new_col = new_col as usize;

                    if maze[new_row][new_col] != '#' {
                        neighbors.push(Edge {
                            node: (new_row, new_col, dir_index),
                            weight: 1,
                        });
                    }
                }

                let clockwise_dir = (dir_index + 1) % 4;
                let counterclockwise_dir = (dir_index + 3) % 4;

                neighbors.push(Edge {
                    node: (row, col, clockwise_dir),
                    weight: 1000,
                });

                neighbors.push(Edge {
                    node: (row, col, counterclockwise_dir),
                    weight: 1000,
                });

                graph.insert(current_state, neighbors);
            }
        }
    }

    graph
}

fn main() -> io::Result<()> {
    let maze = fs::read_to_string("input.txt")?;
    let maze: Vec<Vec<char>> = maze.lines().map(|line| line.chars().collect()).collect();

    let mut start = (0, 0, 1); // Start facing East by default
    let mut end = (0, 0);

    for (row, line) in maze.iter().enumerate() {
        for (col, &ch) in line.iter().enumerate() {
            if ch == 'S' {
                start = (row, col, 1);
            } else if ch == 'E' {
                end = (row, col);
            }
        }
    }

    let graph = convert_maze_to_graph(&maze);
    let (distance, predecessors) = dijkstra(&graph, start, end);

    match distance {
        Some(distance) => {
            println!("Part1 Lowest score: {}", distance);
            println!(
                "Part2 Number of tiles: {}",
                find_best_path_tiles(&predecessors, end).len()
            );
        }
        None => println!("No path found to the end."),
    }

    Ok(())
}
