use std::collections::{VecDeque, HashSet};
use std::vec;

const WALL: char = '#';
const START: char = 'S';

fn calc_distances(grid: &Vec<Vec<char>>, start: (usize, usize)) -> Vec<Vec<isize>> {
    let directions: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, 1), (0, -1)];

    let mut queue = VecDeque::new();

    let rows = grid.len();
    let cols = grid[0].len();

    let mut distances: Vec<Vec<isize>> = vec![vec![-1; cols]; rows];
    distances[start.0][start.1] = 0;

    queue.push_back(start);

    while let Some(current) = queue.pop_front() {
        let (row, col) = current;

        for &(dr, dc) in directions.iter() {
            let new_row = row as isize + dr;
            let new_col = col as isize + dc;

            if new_row >= 0 && new_row < rows as isize && new_col >= 0 && new_col < cols as isize {
                let new_row = new_row as usize;
                let new_col = new_col as usize;

                if grid[new_row][new_col] == WALL || distances[new_row][new_col] != -1 {
                    continue;
                }

                distances[new_row][new_col] = distances[row][col] + 1;
                queue.push_back((new_row, new_col));
            }
        }
    }

    distances
}

fn find_start(grid: &Vec<Vec<char>>) -> Option<(usize, usize)> {
    for (i, row) in grid.iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            if grid[i][j] == START {
                return Some((i, j));
            }
        }
    }
    None
}

fn solve_part1(distances: &Vec<Vec<isize>>, grid: &Vec<Vec<char>>) {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut count = 0;
    for row in 0..rows {
        for col in 0..cols {
            if grid[row][col] == WALL {
                continue;
            }
            for &(dr, dc) in [(-2, 0), (2, 0), (0, 2), (0, -2)].iter() {
                let new_row = row as isize + dr;
                let new_col = col as isize + dc;

                if new_row >= 0
                    && new_row < rows as isize
                    && new_col >= 0
                    && new_col < cols as isize
                {
                    let new_row = new_row as usize;
                    let new_col = new_col as usize;

                    if grid[new_row][new_col] == WALL {
                        continue;
                    }

                    if distances[new_row][new_col] - distances[row][col] >= 102 {
                        count += 1;
                    }
                }
            }
        }
    }

    println!("Total amount of skips for 2 picoseconds: {}", count);
}

fn solve_part2(distances: &Vec<Vec<isize>>, grid: &Vec<Vec<char>>) {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut count = 0;

    for row in 0..rows {
        for col in 0..cols {
            if grid[row][col] == WALL {
                continue;
            }

            for range in 2..=20 {
                for step_r in 0..=range {

                    let step_c = range as isize - step_r as isize;
                    let directions: HashSet<(isize, isize)> = [
                        (row as isize - step_r as isize, col as isize - step_c as isize),
                        (row as isize + step_r as isize, col as isize + step_c as isize),
                        (row as isize + step_r as isize, col as isize - step_c as isize),
                        (row as isize - step_r as isize, col as isize + step_c as isize),
                    ].iter().cloned().collect();

                    for &(new_row, new_col) in directions.iter()
                    {
                        if new_row >= 0 && new_row < rows as isize && new_col >= 0 && new_col < cols as isize {
                            let new_row = new_row as usize;
                            let new_col = new_col as usize;

                            if grid[new_row][new_col] == WALL {
                                continue;
                            }

                            if distances[new_row][new_col] - distances[row][col] >= range as isize + 100 {
                                count += 1;
                            }
                        }
                    }
                }
            }
        }
    }

    println!("Total amount of skips between 2 - 20 picoseconds: {}", count);
}

fn main() -> std::io::Result<()> {
    let input = std::fs::read_to_string("input.txt")?;

    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let start = find_start(&grid).expect("Start position is missing");
    let distances = calc_distances(&grid, start);

    solve_part1(&distances, &grid);
    solve_part2(&distances, &grid); 

    Ok(())
}
