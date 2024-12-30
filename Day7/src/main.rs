use std::{collections::HashSet, fs::File, io::{self, Read}};

fn concat(a: u64, b: u64) -> u64 {
    let result = a;
    let mut multiplier = 1;
    let mut temp = b;
    while temp > 0 {
        multiplier *= 10;
        temp /= 10;
    }
    result * multiplier + b
}

fn can_produce_target(numbers: &[u64], target: u64, include_concat: bool) -> bool {
    let mut dp: HashSet<u64> = HashSet::new();
    dp.insert(numbers[0]); 

    for &num in &numbers[1..] {
        let mut next_dp = HashSet::new();
        for &value in &dp {
            next_dp.insert(value + num);
            next_dp.insert(value * num);
            if include_concat {
                next_dp.insert(concat(value, num));
            }
        }
        dp = next_dp; 
    }

    dp.contains(&target) 
}

fn solve(input: &str, include_concat: bool) -> u64 {
    let mut total_calibration_result = 0;

    for line in input.lines().filter(|l| !l.trim().is_empty()) {
        let parts: Vec<&str> = line.split(':').collect();

        let target: u64 = parts[0].trim().parse().expect("Invalid target number");
        let numbers: Vec<u64> = parts[1]
            .trim()
            .split_whitespace()
            .map(|n| n.parse().expect("Invalid number"))
            .collect();

        if can_produce_target(&numbers, target, include_concat) {
            total_calibration_result += target;
        }
    }

    total_calibration_result
}

fn main() -> io::Result<()>{
    let mut input = String::new();
    File::open("input.txt")?.read_to_string(&mut input)?;

    println!("Part1 solution: {}", solve(&input, false));
    println!("Part2 solution: {}", solve(&input, true));

    Ok(())
}
