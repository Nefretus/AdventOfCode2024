use std::{
    fs::File,
    io::{self, BufReader, Read},
};

fn is_valid_xmas(
    grid: &[Vec<char>],
    r: usize,
    c: usize,
    dr1: isize,
    dc1: isize,
    dr2: isize,
    dc2: isize,
) -> bool {
    let positions = [
        (r as isize + dr1, c as isize + dc1),
        (r as isize + dr2, c as isize + dc2),
    ];
    if !positions.iter().all(|&(nr, nc)| {
        nr >= 0 && nc >= 0 && nr < grid.len() as isize && nc < grid[0].len() as isize
    }) {
        return false;
    }

    let chars: Vec<char> = positions
        .iter()
        .map(|&(nr, nc)| grid[nr as usize][nc as usize])
        .collect();

    (chars[0] == 'M' && chars[1] == 'S') || (chars[0] == 'S' && chars[1] == 'M')
}

fn solve_part1(grid: &[Vec<char>]) {
    let directions = [
        (0, 1),   // Right
        (1, 0),   // Down
        (1, 1),   // Down-Right
        (1, -1),  // Down-Left
        (0, -1),  // Left
        (-1, 0),  // Up
        (-1, -1), // Up-Left
        (-1, 1),  // Up-Right
    ];

    let word = "XMAS";
    let rows = grid.len();
    let cols = grid[0].len();
    let mut count = 0;

    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] != word.chars().next().unwrap() {
                continue;
            }

            for &(dr, dc) in &directions {
                let mut found = true;

                for (i, ch) in word.chars().enumerate() {
                    let nr = r as isize + i as isize * dr;
                    let nc = c as isize + i as isize * dc;

                    if nr < 0 || nc < 0 || nr >= rows as isize || nc >= cols as isize {
                        found = false;
                        break;
                    }

                    if grid[nr as usize][nc as usize] != ch {
                        found = false;
                        break;
                    }
                }

                if found {
                    count += 1;
                }
            }
        }
    }

    println!("Part 1 solution: {}", count);
}
fn solve_part2(grid: &Vec<Vec<char>>) {
    let directions_part2 = [
        ((-1, -1), (1, 1)), // bottom left - top right
        ((-1, 1), (1, -1)), // bottom left - bottom right
    ];

    let rows = grid.len();
    let cols = grid[0].len();
    let mut count = 0;
    
    for r in 1..rows - 1 {
        for c in 1..cols - 1 {
            if grid[r][c] == 'A' {
                let mut found_match = true;
                for &((dr1, dc1), (dr2, dc2)) in &directions_part2 {
                    if !is_valid_xmas(&grid, r, c, dr1, dc1, dr2, dc2) {
                        found_match = false;
                    }
                }
                if found_match {
                    count += 1;
                }
            }
        }
    }
    
    println!("Part 2 solution: {}", count);
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    BufReader::new(File::open("input.txt")?).read_to_string(&mut input)?;

    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    solve_part1(&grid);
    solve_part2(&grid);

    Ok(())
}
