use std::collections::HashSet;
use std::fs;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn turn_right(self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn move_forward(self, position: (usize, usize)) -> Option<(usize, usize)> {
        match self {
            Direction::Up => {
                if position.0 == 0 {
                    None
                } else {
                    Some((position.0 - 1, position.1))
                }
            }
            Direction::Down => Some((position.0 + 1, position.1)),
            Direction::Left => {
                if position.1 == 0 {
                    None
                } else {
                    Some((position.0, position.1 - 1))
                }
            }
            Direction::Right => Some((position.0, position.1 + 1)),
        }
    }
}

fn solve_part1(map: &Vec<Vec<char>>, start_direction: Direction, start_position: (usize, usize)) {
    let map_height = map.len();
    let map_width = map[0].len();

    let mut direction = start_direction;
    let mut position = start_position;

    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    visited.insert(start_position);

    loop {
        if let Some(next_position) = direction.move_forward(position) {
            if next_position.0 >= map_height || next_position.1 >= map_width {
                break;
            }
            if map[next_position.0][next_position.1] == '#' {
                direction = direction.turn_right();
            } else {
                position = next_position;
                visited.insert(position);
            }
        } else {
            break;
        }
    }

    println!("Distinct positions visited: {}", visited.len());
}

fn solve_part2(map: &Vec<Vec<char>>, start_direction: Direction, start_position: (usize, usize)) {
    println!("Solving part2...");

    let simulate = |map: &Vec<Vec<char>>, obstruction: Option<(usize, usize)>| -> bool {
        let mut visited = HashSet::new();
        let mut position = start_position;
        let mut direction = start_direction;

        loop {
            if visited.contains(&(position, direction)) {
                return true;
            }
            visited.insert((position, direction));
            if let Some(next_position) = direction.move_forward(position) {
                if next_position.0 >= map.len() || next_position.1 >= map[0].len() {
                    break;
                }
                if map[next_position.0][next_position.1] == '#'
                    || obstruction == Some(next_position)
                {
                    direction = direction.turn_right();
                } else {
                    position = next_position;
                }
            } else {
                break;
            }
        }

        false
    };

    let mut valid_obstructions = HashSet::new();
    for row in 0..map.len() {
        for col in 0..map[0].len() {
            if map[row][col] == '.' && (row, col) != start_position {
                if simulate(&map, Some((row, col))) {
                    valid_obstructions.insert((row, col));
                }
            }
        }
    }

    println!(
        "Number of valid obstruction positions: {}",
        valid_obstructions.len()
    );
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to read input file");
    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut start_position = (0, 0);
    let mut start_direction = Direction::Up;

    for (row, line) in map.iter().enumerate() {
        for (col, &cell) in line.iter().enumerate() {
            if "^v<>".contains(cell) {
                start_position = (row, col);
                start_direction = match cell {
                    '^' => Direction::Up,
                    'v' => Direction::Down,
                    '<' => Direction::Left,
                    '>' => Direction::Right,
                    _ => panic!("Invalid guard direction"),
                };
                break;
            }
        }
    }

    solve_part1(&map, start_direction.clone(), start_position);
    solve_part2(&map, start_direction.clone(), start_position);
}
