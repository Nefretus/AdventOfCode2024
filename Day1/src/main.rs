use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

fn solve_part1(mut left_list: Vec<u32>, mut right_list: Vec<u32>) {
    left_list.sort();
    right_list.sort();

    let dist = left_list
        .iter()
        .zip(right_list.iter())
        .fold(0, |acc, (left, right)| acc + left.abs_diff(*right));

    println!("Part1: {}", dist);
}

fn solve_part2(left_list: &Vec<u32>, right_list: &Vec<u32>) {
    let mut left_map = HashMap::new();
    let mut right_map = HashMap::new();

    let counter_fn = |list: &Vec<u32>, map: &mut HashMap<u32, u32> | {
        for &number in list.iter() {
            *map.entry(number).or_insert(0) += 1;
        }
    };

    counter_fn(left_list, &mut left_map);
    counter_fn(right_list, &mut right_map);

    let dist = left_map.iter().fold(0, |acc, (left_key, left_val)| {
        if let Some(right_val) = right_map.get(left_key) {
            acc + left_key * left_val * right_val
        } else {
            acc
        }
    });

    println!("Part2: {}", dist);
}

fn main() -> std::io::Result<()> {
    let reader: BufReader<File> = BufReader::new(File::open("input.txt")?);
    let mut left_list = vec![];
    let mut right_list = vec![];
    reader.lines().filter_map(|x| x.ok()).for_each(|l| {
        l.split_whitespace().enumerate().for_each(|(idx, x)| {
            if idx & 1 != 0 {
                right_list.push(x.parse::<u32>().expect("Incorrect Location ID found"));
            } else {
                left_list.push(x.parse::<u32>().expect("Incorrect Location ID found"));
            }
        });
    });

    solve_part1(left_list.clone(), right_list.clone());
    solve_part2(&left_list, &right_list);

    Ok(())
}
