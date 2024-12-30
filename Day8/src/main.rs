use std::collections::{HashMap, HashSet};
use std::io::{self};

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
struct Point {
    x: usize,
    y: usize,
}

fn find_antinodes_part1(map_range: Point, antenna_locations: &HashSet<Point>) -> HashSet<Point> {
    let mut antinodes: HashSet<Point> = HashSet::new();
    let locations: Vec<&Point> = antenna_locations.iter().collect();
    for i in 0..locations.len() {
        for j in i + 1..locations.len() {
            let (l1, l2) = (locations[i], locations[j]);

            let left_point = if l1.x < l2.x { l1 } else { l2 };
            let high_point = if l1.y > l2.y { l1 } else { l2 };
            let right_point = if l1.x > l2.x { l1 } else { l2 };
            let low_point = if l1.y < l2.y { l1 } else { l2 };

            let dist_x = right_point.x.abs_diff(left_point.x);
            let dist_y = high_point.y.abs_diff(low_point.y);

            if right_point == low_point {
                if right_point.x + dist_x < map_range.x {
                    if let Some(res_y) = low_point.y.checked_sub(dist_y) {
                        let new_pos = Point {
                            x: right_point.x + dist_x,
                            y: res_y,
                        };
                        antinodes.insert(new_pos);
                    }
                }
                if let Some(res_x) = left_point.x.checked_sub(dist_x) {
                    if high_point.y + dist_y < map_range.y {
                        let new_pos = Point {
                            x: res_x,
                            y: high_point.y + dist_y,
                        };
                        antinodes.insert(new_pos);
                    }
                }
            } else {
                if let Some(res_x) = left_point.x.checked_sub(dist_x) {
                    if let Some(res_y) = low_point.y.checked_sub(dist_y) {
                        let new_pos = Point { x: res_x, y: res_y };
                        antinodes.insert(new_pos);
                    }
                }
                if right_point.x + dist_x < map_range.x {
                    if high_point.y + dist_y < map_range.y {
                        let new_pos = Point {
                            x: right_point.x + dist_x,
                            y: high_point.y + dist_y,
                        };
                        antinodes.insert(new_pos);
                    }
                }
            }
        }
    }

    antinodes
}

fn gcd(a: isize, b: isize) -> isize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn find_antinodes_part2(map_range: Point, antenna_locations: &HashSet<Point>) -> HashSet<Point> {
    let mut antinodes: HashSet<Point> = HashSet::new();
    let locations: Vec<&Point> = antenna_locations.iter().collect();

    if locations.len() > 1 {
        antinodes.extend(antenna_locations.clone());
    }

    for i in 0..locations.len() {
        for j in i + 1..locations.len() {
            let (a, b) = (locations[i], locations[j]);

            let dx = b.x as isize - a.x as isize;
            let dy = b.y as isize - a.y as isize;

            let gcd = gcd(dx.abs(), dy.abs());
            let step_x = dx / gcd;
            let step_y = dy / gcd;

            let mut x = a.x as isize;
            let mut y = a.y as isize;
            while x >= 0 && y >= 0 && x < map_range.x as isize && y < map_range.y as isize {
                antinodes.insert(Point {
                    x: x as usize,
                    y: y as usize,
                });
                x -= step_x;
                y -= step_y;
            }

            let mut x = b.x as isize;
            let mut y = b.y as isize;
            while x >= 0 && y >= 0 && x < map_range.x as isize && y < map_range.y as isize {
                antinodes.insert(Point {
                    x: x as usize,
                    y: y as usize,
                });
                x += step_x;
                y += step_y;
            }
        }
    }

    antinodes
}

fn solve_part1(
    antenna_locations: &HashMap<char, HashSet<Point>>,
    map_width: usize,
    map_height: usize,
) {
    let mut unique_points: HashSet<Point> = HashSet::new();
    for (_, locations) in antenna_locations {
        unique_points.extend(find_antinodes_part1(
            Point {
                x: map_width,
                y: map_height,
            },
            &locations,
        ));
    }
    println!("Part1 solution: {}", unique_points.len());
}

fn solve_part2(
    antenna_locations: &HashMap<char, HashSet<Point>>,
    map_width: usize,
    map_height: usize,
) {
    let mut unique_antinodes: HashSet<Point> = HashSet::new();
    for (_, locations) in antenna_locations {
        unique_antinodes.extend(find_antinodes_part2(
            Point {
                x: map_width,
                y: map_height,
            },
            locations,
        ));
    }
    println!("Part2 solution: {}", unique_antinodes.len());
}

fn main() -> io::Result<()> {
    let input = std::fs::read_to_string("input.txt")?;
    let map: Vec<Vec<char>> = input.lines().map(|c| c.chars().collect()).collect();
    let map_height = map.len();
    let map_width = map[0].len();

    let mut antenna_locations: HashMap<char, HashSet<Point>> = HashMap::new();
    for row in 0..map_height {
        for col in 0..map_width {
            if map[row][col].is_ascii_alphanumeric() {
                antenna_locations
                    .entry(map[row][col])
                    .or_insert_with(HashSet::new)
                    .insert(Point { x: col, y: row });
            }
        }
    }

    solve_part1(&antenna_locations, map_width, map_height);
    solve_part2(&antenna_locations, map_width, map_height);

    Ok(())
}
