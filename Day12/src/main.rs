use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Grid {
    data: Vec<Vec<char>>,
}

impl Grid {
    fn new(data: Vec<Vec<char>>) -> Self {
        Self { data }
    }

    fn get(&self, pos: (usize, usize)) -> Option<char> {
        let (row, col) = pos;
        if row < self.data.len() && col < self.data[0].len() {
            Some(self.data[row][col])
        } else {
            None
        }
    }

    fn iter_directions_full(
        &self,
        (row, col): (usize, usize),
    ) -> Vec<(Direction, Option<(usize, usize)>)> {
        vec![
            (
                Direction::Up,
                if row > 0 { Some((row - 1, col)) } else { None },
            ),
            (
                Direction::Down,
                if row + 1 < self.data.len() {
                    Some((row + 1, col))
                } else {
                    None
                },
            ),
            (
                Direction::Left,
                if col > 0 { Some((row, col - 1)) } else { None },
            ),
            (
                Direction::Right,
                if col + 1 < self.data[0].len() {
                    Some((row, col + 1))
                } else {
                    None
                },
            ),
        ]
    }

    fn iter_directions(&self, (row, col): (usize, usize)) -> Vec<(usize, usize)> {
        let mut neighbors = Vec::new();
        if row > 0 {
            neighbors.push((row - 1, col));
        }
        if row + 1 < self.data.len() {
            neighbors.push((row + 1, col));
        }
        if col > 0 {
            neighbors.push((row, col - 1));
        }
        if col + 1 < self.data[0].len() {
            neighbors.push((row, col + 1));
        }
        neighbors
    }
}

fn solve(garden: &Grid, plant: char) -> (u32, u32) {
    let mut total_area = 0;
    let mut total_sides = 0;

    let mut visited = HashSet::new();

    for (row_idx, row) in garden.data.iter().enumerate() {
        for (col_idx, &cell) in row.iter().enumerate() {
            if cell != plant || visited.contains(&(row_idx, col_idx)) {
                continue;
            }

            let mut area: u32 = 0;
            let mut perimeter: u32 = 0;
            let mut sides = 0;
            let mut queue = VecDeque::new();
            let mut fence_sides: HashMap<Direction, HashSet<(usize, usize)>> = HashMap::new();

            queue.push_back((row_idx, col_idx));

            while let Some(current_pos) = queue.pop_front() {
                if visited.contains(&current_pos) {
                    continue;
                }
                
                visited.insert(current_pos);
                area += 1;

                for (direction, neighbor_pos) in garden.iter_directions_full(current_pos) {
                    if let Some(neighbor) = neighbor_pos {
                        if garden.get(neighbor) == Some(plant) {
                            queue.push_back(neighbor);
                            continue;
                        }
                    }
                    perimeter += 1;
                    fence_sides
                        .entry(direction)
                        .or_default()
                        .insert(current_pos);
                }
            }

            for fence in fence_sides.values() {
                let mut side_visited = HashSet::new();

                for &pos in fence {
                    if side_visited.contains(&pos) {
                        continue;
                    }

                    sides += 1;

                    let mut side_queue = VecDeque::new();
                    side_queue.push_back(pos);

                    while let Some(side_pos) = side_queue.pop_front() {
                        if side_visited.contains(&side_pos) {
                            continue;
                        }
                        side_visited.insert(side_pos);

                        for neighbor in garden.iter_directions(side_pos) {
                            if fence.contains(&neighbor) {
                                side_queue.push_back(neighbor);
                            }
                        }
                    }
                }
            }

            total_area += area * perimeter;
            total_sides += area * sides;
        }
    }

    (total_area, total_sides)
}

fn main() -> std::io::Result<()> {
    let input_str = fs::read_to_string("input.txt")?;

    let garden: Vec<Vec<char>> = input_str
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let grid = Grid::new(garden);

    let unique_plants: HashSet<char> = grid
        .data
        .iter()
        .flat_map(|row| row.iter())
        .copied()
        .collect();

    let (standard_price, discount_price) =
        unique_plants.iter().map(|&plant| solve(&grid, plant)).fold(
            (0, 0),
            |(acc_standard, acc_discount), (standard, discount)| {
                (acc_standard + standard, acc_discount + discount)
            },
        );

    println!(
        "Standard Price: {}, Discount Price: {}",
        standard_price, discount_price
    );

    Ok(())
}
