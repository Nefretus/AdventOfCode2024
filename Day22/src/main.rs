use std::collections::{HashMap, HashSet};

fn mix(secret_num: usize, calc_value: usize) -> usize {
    calc_value ^ secret_num
}

fn prune(secret_num: usize) -> usize {
    secret_num % 16_777_216
}

fn produce_secret_num(secret_num: usize) -> usize {
    let step1 = prune(mix(secret_num, secret_num * 64));
    let step2 = prune(mix(step1, step1 / 32));
    prune(mix(step2, step2 * 2048))
}

fn solve(input: &str, iterations: usize) {
    let mut seq_profit: HashMap<(isize, isize, isize, isize), usize> = HashMap::new();

    let total_sum: usize = input
        .lines()
        .filter_map(|line| line.trim().parse::<usize>().ok())   
        .map(|mut secret_num| {
            let mut visited = HashSet::new();
            let mut prices = Vec::with_capacity(iterations + 1);

            prices.push(secret_num % 10);
            (0..iterations).for_each(|_| {
                secret_num = produce_secret_num(secret_num);
                prices.push(secret_num % 10);
            });

            for window in prices.windows(5) {
                let [a, b, c, d, e] = [window[0], window[1], window[2], window[3], window[4]];
                let seq = (
                    b as isize - a as isize,
                    c as isize - b as isize,
                    d as isize - c as isize,
                    e as isize - d as isize,
                );
                if visited.insert(seq) {
                    *seq_profit.entry(seq).or_insert(0) += e as usize;
                }
            }

            secret_num
        })
        .sum();

    let max_profit = seq_profit.values().max().unwrap_or(&0);

    println!("Sum of secret nums after {} iterations: {}", iterations, total_sum);
    println!("Bananas bought with the best sequence: {}", max_profit);
}

fn main() -> std::io::Result<()> {
    let input = std::fs::read_to_string("input.txt")?;

    solve(&input, 2000);

    Ok(())
}
