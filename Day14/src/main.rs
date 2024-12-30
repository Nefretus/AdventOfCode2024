use itertools::Itertools;
use std::{fs, io};

#[derive(Debug)]
struct Robot {
    pos_x: usize,
    pos_y: usize,
    vel_x: isize,
    vel_y: isize,
}

fn simulate(robots: &mut Vec<Robot>, width: isize, height: isize) {
    for robot in robots {
        let new_x = (robot.pos_x as isize + robot.vel_x).rem_euclid(width);
        let new_y = (robot.pos_y as isize + robot.vel_y).rem_euclid(height);

        robot.pos_x = new_x as usize;
        robot.pos_y = new_y as usize;
    }
}

fn tree_detected(robots: &Vec<Robot>) -> bool {
    robots
        .iter()
        .map(|robot| (robot.pos_x, robot.pos_y))
        .all_unique()
}

fn calculate_safety_factor(
    mut robots: Vec<Robot>,
    width: isize,
    height: isize,
    steps: usize,
) -> (usize, usize) {
    for _ in 0..steps {
        simulate(&mut robots, width, height);
    }

    let mut quadrant_counts = [0; 4];
    for robot in &robots {
        let x = robot.pos_x as isize;
        let y = robot.pos_y as isize;
        if x == width / 2 || y == height / 2 {
            continue;
        }

        let quadrant = match (x < width / 2, y < height / 2) {
            (true, true) => 0,
            (false, true) => 1,
            (true, false) => 2,
            (false, false) => 3,
        };

        quadrant_counts[quadrant] += 1;
    }

    let mut christmas_tree_event_it = steps;
    while !tree_detected(&robots) {
        simulate(&mut robots, width, height);
        christmas_tree_event_it += 1;
    }
    render_grid(&robots, width as usize, height as usize);

    (quadrant_counts.iter().product(), christmas_tree_event_it)
}

fn render_grid(robots: &Vec<Robot>, width: usize, height: usize) {
    let mut grid = vec![vec![0; width]; height];

    for robot in robots {
        grid[robot.pos_y][robot.pos_x] += 1;
    }

    for row in grid {
        println!(
            "{}",
            row.iter()
                .map(|&count| if count == 0 {
                    '.'
                } else {
                    char::from_digit(count as u32, 10).unwrap_or('9')
                })
                .collect::<String>()
        );
    }
    println!();
}

fn main() -> io::Result<()> {
    let input = fs::read_to_string("input.txt")?;

    let width: usize = 101;
    let height: usize = 103;

    let robots: Vec<Robot> = input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();

            let pos: Vec<usize> = parts[0][2..]
                .split(',')
                .map(|x| x.parse::<usize>().unwrap())
                .collect();

            let vel: Vec<isize> = parts[1][2..]
                .split(',')
                .map(|x| x.parse::<isize>().unwrap())
                .collect();

            Robot {
                pos_x: pos[0],
                pos_y: pos[1],
                vel_x: vel[0],
                vel_y: vel[1],
            }
        })
        .collect();

    let iterations = 100;
    let (safety_factor, christmas_tree_event) =
        calculate_safety_factor(robots, width as isize, height as isize, iterations);

    println!(
        "Safety factor: {} Christams tree after: {} iterations",
        safety_factor, christmas_tree_event
    );

    Ok(())
}
