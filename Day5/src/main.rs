use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufReader, Read};

fn main() -> io::Result<()> {
    let solve_part2 = true;
    let input_file = "input.txt";
    let mut input = String::new();

    let mut rules: HashMap<i32, HashSet<i32>> = HashMap::new();
    BufReader::new(File::open(input_file)?).read_to_string(&mut input)?;

    let mut pages_idx = 0;

    for (idx, line) in input.lines().enumerate() {
        if line.is_empty() {
            pages_idx = idx;
            break;
        }
        let (start, end): (i32, i32) = line
            .split_once('|')
            .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
            .unwrap();
        rules.entry(start).or_insert_with(HashSet::new).insert(end);
    }

    let mut banned_pages_indices: HashSet<usize> = HashSet::new();
    let mut updates: Vec<Vec<i32>> = vec![];

    for (page_idx, page_list) in input.lines().skip(pages_idx + 1).enumerate() {
        let page_vec: Vec<i32> = page_list
            .split(',')
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        updates.push(page_vec);

        let mut is_banned = false;
        for (idx, &page) in updates[page_idx].iter().enumerate() {
            if let Some(banned_rules) = rules.get(&page) {
                if updates[page_idx][..idx]
                    .iter()
                    .any(|prev_val| banned_rules.contains(prev_val))
                {
                    is_banned = true;
                    break;
                }
            }
        }

        if is_banned {
            banned_pages_indices.insert(page_idx);
        }
    }

    if !solve_part2 {
        let mut total_middle_sum = 0;

        for (idx, page_list) in updates.iter().enumerate() {
            if !banned_pages_indices.contains(&idx) {
                total_middle_sum += page_list[page_list.len() / 2];
            }
        }

        println!("{}", total_middle_sum);
    } else {
        let mut total_middle_sum = 0;

        for idx in banned_pages_indices {
            let mut page_vec = updates[idx].clone();
            page_vec.sort_by(|a, b| {
                if rules.get(a).map_or(false, |rule| rule.contains(b)) {
                    std::cmp::Ordering::Less
                } else if rules.get(b).map_or(false, |rule| rule.contains(a)) {
                    std::cmp::Ordering::Greater
                } else {
                    std::cmp::Ordering::Equal
                }
            });
            
            total_middle_sum += page_vec[page_vec.len() / 2];
        }

        println!("{}", total_middle_sum);
    }

    Ok(())
}
