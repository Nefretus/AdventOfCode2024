use std::collections::HashMap;
use std::fs;
use std::io;

fn count_combinations<'a>(
    design: &'a str,
    towels: &Vec<&'a str>,
    memo: &mut HashMap<&'a str, usize>,
) -> usize {
    if design.is_empty() {
        return 1;
    }

    if let Some(&result) = memo.get(design) {
        return result;
    }

    let mut total_count = 0;

    for &towel in towels {
        if design.starts_with(towel) {
            let remaining = &design[towel.len()..];
            total_count += count_combinations(remaining, towels, memo);
        }
    }

    memo.insert(design, total_count);

    total_count
}

fn solve(towels: &Vec<&str>, patterns: &Vec<&str>) {
    let mut total_combination_count = 0;
    let mut possible_count = 0;

    for &pattern in patterns {
        let mut memo = HashMap::new();
        let combination_count = count_combinations(pattern, towels, &mut memo);

        if combination_count > 0 {
            possible_count += 1;
        }

        total_combination_count += combination_count;
    }

    println!(
        "Number of possible designs: {}\nNumber of possible combinations: {}",
        possible_count, total_combination_count
    );
}

fn main() -> io::Result<()> {
    let input = fs::read_to_string("input.txt")?;
    let mut lines = input.lines();
    let mut towels = Vec::new();

    while let Some(line) = lines.next() {
        if line.trim().is_empty() {
            break;
        }
        towels.extend(line.split(',').map(|item| item.trim()));
    }

    solve(&towels, &lines.collect());

    Ok(())
}
