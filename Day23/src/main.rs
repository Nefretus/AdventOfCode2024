use std::collections::{HashMap, HashSet};

fn build_graph(input: &str) -> HashMap<&str, HashSet<&str>> {
    let mut graph = HashMap::new();

    input.lines().for_each(|line| {
        let parts: Vec<_> = line.trim().split('-').collect();
        graph.entry(parts[0])
            .or_insert_with(HashSet::new)
            .insert(parts[1]);
        graph.entry(parts[1])
            .or_insert_with(HashSet::new)
            .insert(parts[0]);
    });

    graph
}

fn solve_part1(graph: &HashMap<&str, HashSet<&str>>) {
    let mut conn_computers: HashSet<(&str, &str, &str)> = HashSet::new();

    for (&node1, neighbors1) in graph {
        for &node2 in neighbors1 {
            if let Some(neighbors2) = graph.get(node2) {
                for &node3 in neighbors2 {
                    if node1 != node3 && graph.get(node3).unwrap_or(&HashSet::new()).contains(node1) {
                        let mut group = vec![node1, node2, node3];
                        group.sort();
                        if group.iter().any(|&node| node.starts_with('t')) {
                            conn_computers.insert((group[0], group[1], group[2]));
                        }
                    }
                }
            }
        }
    }

    println!("Count of unique triplets: {}", conn_computers.len());
}

fn find_connections<'a>(node: &str, curr_group: &mut HashSet<&'a str>, groups: &mut HashSet<Vec<&'a str>>, graph: &HashMap<&'a str, HashSet<&'a str>>) {
    let mut sorted: Vec<&str> = curr_group.iter().cloned().collect();
    sorted.sort();
    if groups.contains(&sorted) {
        return;
    }
    groups.insert(sorted);
    if let Some(neighbors) = graph.get(node) {
        for neighbor in neighbors {
            if *neighbor == node {
                continue; 
            }
            if !curr_group.iter().all(|x| graph.get(neighbor).unwrap_or(&HashSet::new()).contains(x)) {
                continue;
            }
            if !curr_group.contains(neighbor) {
                curr_group.insert(neighbor);
                find_connections(neighbor, curr_group, groups, graph);
                curr_group.remove(neighbor);
            }
        }
    }
}

fn solve_part2(graph: &HashMap<&str, HashSet<&str>>) {
    let mut groups: HashSet<Vec<&str>> = HashSet::new();

    for node in graph.keys() {
        let mut curr_group: HashSet<&str> = HashSet::new();
        curr_group.insert(node);
        find_connections(node, &mut curr_group, &mut groups, graph);
    }
    
    if let Some(largest_group) = groups.iter().max_by_key(|group| group.len()) {
        println!("{:?}", largest_group.join(","));
    }
}

fn main() -> std::io::Result<()> {
    let input = std::fs::read_to_string("input.txt")?;

    let graph = build_graph(&input);

    solve_part1(&graph);
    solve_part2(&graph);

    Ok(())
}
