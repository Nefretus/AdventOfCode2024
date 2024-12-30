use std::{
    collections::HashSet,
    fs::File,
    io::{self, BufRead, BufReader},
};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Point {
    x: usize,
    y: usize,
}

const END_NODE: u32 = 9;

fn find_route_part1(
    curr_pos: &Point,
    map: &Vec<Vec<u32>>,
    visited: &mut HashSet<Point>,
    found_nines: &mut HashSet<Point>,
    current_height: u32,
) {
    let directions: &[(i32, i32)] = &[(-1, 0), (1, 0), (0, 1), (0, -1)];

    if map[curr_pos.x][curr_pos.y] == END_NODE {
        found_nines.insert(*curr_pos);
        return;
    }

    for dir in directions {
        let new_x = curr_pos.x as i32 + dir.0;
        let new_y = curr_pos.y as i32 + dir.1;

        if new_x >= 0 && new_y >= 0 {
            let new_x = new_x as usize;
            let new_y = new_y as usize;

            if new_x < map.len() && new_y < map[0].len() {
                let new_pos = Point { x: new_x, y: new_y };
                if !visited.contains(&new_pos) && map[new_x][new_y] == current_height + 1 {
                    visited.insert(new_pos);
                    find_route_part1(&new_pos, map, visited, found_nines, current_height + 1);
                    visited.remove(&new_pos);
                }
            }
        }
    }
}

fn find_route_part2(
    curr_pos: &Point,
    map: &Vec<Vec<u32>>,
    visited: &mut HashSet<Point>,
    found_nines: &mut u32,
    current_height: u32,
) {
    let directions: &[(i32, i32)] = &[(-1, 0), (1, 0), (0, 1), (0, -1)];

    if map[curr_pos.x][curr_pos.y] == END_NODE {
        *found_nines += 1;
        return;
    }

    for dir in directions {
        let new_x = curr_pos.x as i32 + dir.0;
        let new_y = curr_pos.y as i32 + dir.1;

        if new_x >= 0 && new_y >= 0 {
            let new_x = new_x as usize;
            let new_y = new_y as usize;

            if new_x < map.len() && new_y < map[0].len() {
                let new_pos = Point { x: new_x, y: new_y };
                if !visited.contains(&new_pos) && map[new_x][new_y] == current_height + 1 {
                    visited.insert(new_pos);
                    find_route_part2(&new_pos, map, visited, found_nines, current_height + 1);
                    visited.remove(&new_pos);
                }
            }
        }
    }
}

fn solve_part1(input: &Vec<Vec<u32>>, start_positions: &Vec<Point>) {
    let mut total_score = 0;

    for start in start_positions {
        let mut visited = HashSet::new();
        let mut found_nines = HashSet::new();

        visited.insert(*start);
        find_route_part1(start, &input, &mut visited, &mut found_nines, 0);

        total_score += found_nines.len();
    }

    println!("Part1 score of all trailheads: {}", total_score);
}

fn solve_part2(input: &Vec<Vec<u32>>, start_positions: &Vec<Point>) {
    let mut total_score = 0;

    for start in start_positions {
        let mut visited = HashSet::new();
        let mut found_nines: u32 = 0;

        visited.insert(*start);
        find_route_part2(start, &input, &mut visited, &mut found_nines, 0);

        total_score += found_nines;
    }

    println!("Part2 score of all trailheads: {}", total_score);
}

fn main() -> io::Result<()> {
    let reader = BufReader::new(File::open("input.txt")?);

    let input: Vec<Vec<u32>> = reader
        .lines()
        .flatten()
        .map(|line| line.chars().filter_map(|c| c.to_digit(10)).collect())
        .collect();

    let start_positions: Vec<Point> = input
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter().enumerate().filter_map(move |(j, &value)| {
                if value == 0 {
                    Some(Point { x: i, y: j })
                } else {
                    None
                }
            })
        })
        .collect();

    solve_part1(&input, &start_positions);
    solve_part2(&input, &start_positions);

    Ok(())
}
