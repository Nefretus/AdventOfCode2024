use std::collections::{HashMap, VecDeque};
use std::fs;

fn solve_part1(mut stones: VecDeque<u64>, iterations: u32) {
    for _ in 0..iterations {
        let mut new_stones = VecDeque::new();

        while let Some(stone) = stones.pop_front() {
            if stone == 0 {
                new_stones.push_back(1);
            } else {
                let stone_str = stone.to_string();
                let len = stone_str.len();

                if len % 2 == 0 {
                    let divisor = 10u64.pow((len / 2) as u32);
                    let left = stone / divisor;
                    let right = stone % divisor;
                    new_stones.push_back(left);
                    new_stones.push_back(right);
                } else {
                    new_stones.push_back(stone * 2024);
                }
            }
        }
        
        stones = new_stones;
    }

    println!(
        "Number of stones after {} iterations: {:?}",
        iterations,
        stones.len()
    );
}

fn count_stones(stone: u64, iterations: u32, cache: &mut HashMap<(u64, u32), u64>) -> u64 {
    if let Some(&count) = cache.get(&(stone, iterations)) {
        return count;
    }

    if iterations == 0 {
        return 1;
    }

    let result = if stone == 0 {
        count_stones(1, iterations - 1, cache)
    } else {
        let stone_str = stone.to_string();
        let len = stone_str.len();
        if len % 2 == 0 {
            let divisor = 10u64.pow((len / 2) as u32);
            count_stones(stone / divisor, iterations - 1, cache)
                + count_stones(stone % divisor, iterations - 1, cache)
        } else {
            count_stones(stone * 2024, iterations - 1, cache)
        }
    };

    cache.insert((stone, iterations), result);

    result
}

fn solve_part2(stones: &VecDeque<u64>, iterations: u32) {
    let mut cache: HashMap<(u64, u32), u64> = HashMap::new();
    let count: u64 = stones
        .iter()
        .map(|stone| count_stones(*stone, 75, &mut cache))
        .sum();
    println!(
        "Number of stones after {} iterations: {}",
        iterations, count
    );
}

fn main() -> std::io::Result<()> {
    let buf = fs::read_to_string("input.txt")?;
    let stones: VecDeque<u64> = buf
        .split_whitespace()
        .map(|x| x.parse::<u64>().expect("Invalid input data"))
        .collect();

    solve_part1(stones.clone(), 25);
    solve_part2(&stones, 75);
    Ok(())
}
