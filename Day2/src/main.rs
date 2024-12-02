use std::{
    fs::File,
    io::{BufRead, BufReader},
    ops::ControlFlow,
};

#[derive(Debug)]
enum Monotonicity {
    Increase,
    Decrease,
    None,
}

const MAX_DIFFER: i32 = 3;
const MIN_DIFFER: i32 = 1;

fn is_within_range(diff: i32) -> bool {
    (MIN_DIFFER..=MAX_DIFFER).contains(&diff)
}

fn verify(v: &[i32]) -> Option<()> {
    let mut monocity = Monotonicity::None;

    for window in v.windows(2) {
        let (prev, curr) = (window[0], window[1]);
        let diff = (prev - curr).abs();

        if !is_within_range(diff) {
            return None;
        }

        monocity = match monocity {
            Monotonicity::None => {
                if curr > prev {
                    Monotonicity::Increase
                } else {
                    Monotonicity::Decrease
                }
            }
            Monotonicity::Increase if curr < prev => return None,
            Monotonicity::Decrease if curr > prev => return None,
            _ => monocity,
        };
    }

    Some(())
}

fn solve_part1_and_part2(input: &[Vec<i32>], solve_part2: bool) -> u32 {
    let mut valid_levels = 0;

    for v in input {
        match verify(v.as_slice()) {
            Some(()) => {
                valid_levels += 1;
            }
            None if solve_part2 => {
                // Problem Dampener: Try removing one element
                if (0..v.len()).any(|skip_idx| {
                    let modified = v
                        .iter()
                        .enumerate()
                        .filter_map(|(idx, &val)| if idx != skip_idx { Some(val) } else { None })
                        .collect::<Vec<_>>();
                    matches!(verify(&modified), Some(()))
                }) {
                    valid_levels += 1;
                }
            }
            _ => {}
        }
    }

    valid_levels
}

fn main() -> std::io::Result<()> {
    let solve_part2 = true;

    let reader = BufReader::new(File::open("input.txt")?);
    let input: Vec<Vec<i32>> = reader
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    let valid_levels = solve_part1_and_part2(&input, solve_part2);

    println!("Valid levels: {}", valid_levels);
    Ok(())
}
