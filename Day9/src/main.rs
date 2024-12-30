use std::{fs, io, iter, usize};

const FREE_SPACE: i32 = -1;

fn solve_part1(values: &Vec<usize>) {
    let mut expanded_list: Vec<i32> = Vec::with_capacity(values.iter().sum());
    for i in (0..values.len()).step_by(2) {
        let file_size = values[i];
        expanded_list.extend(iter::repeat(i as i32 / 2).take(file_size));
        if i + 1 < values.len() {
            let free_space = values[i + 1];
            expanded_list.extend(iter::repeat(FREE_SPACE).take(free_space));
        }
    }

    let mut left = 0;
    let mut right = expanded_list.len() - 1;

    while left <= right {
        if expanded_list[left] == FREE_SPACE {
            if expanded_list[right] == FREE_SPACE {
                right -= 1;
            } else {
                expanded_list[left] = expanded_list[right];
                expanded_list[right] = FREE_SPACE;
                left += 1;
                right -= 1;
            }
        } else {
            left += 1;
        }
    }

    let checksum: i64 = expanded_list
        .iter()
        .enumerate()
        .filter(|&(_, &value)| value != FREE_SPACE)
        .map(|(position, &value)| position as i64 * value as i64)
        .sum();

    println!("Part1 checksum: {}", checksum);
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct FileDesc {
    id: usize,
    file_size: usize,
    free_space: usize,
    current_left: usize,
    moved: bool,
}

// this part2 is terrible, will be back with better approach
fn solve_part2(values: &Vec<usize>) {
    let mut pairs: Vec<FileDesc> = vec![];
    for i in (0..values.len()).step_by(2) {
        let file_size = values[i];
        let free_space = if i + 1 < values.len() {
            values[i + 1]
        } else {
            0
        };
        pairs.push(FileDesc {
            id: i / 2,
            file_size,
            free_space,
            current_left: if i != 0 { (i / 2) - 1 } else { usize::MAX },
            moved: false,
        });
    }

    let mut last_e = pairs.len() - 1;
    for _ in 0..pairs.len() - 1 {
        let mut left = 0;
        let right = last_e;
        let mut found_change = false;

        while left < right {
            if pairs[left].free_space >= pairs[right].file_size && !pairs[right].moved {
                for id_based_on_idx in 0..pairs.len() {
                    if pairs[id_based_on_idx].id == pairs[right].current_left {
                        pairs[id_based_on_idx].free_space +=
                            pairs[right].file_size + pairs[right].free_space;
                    }
                }

                pairs[right].free_space = pairs[left].free_space - pairs[right].file_size;
                pairs[left].free_space = 0;

                for j in 0..pairs.len() {
                    if pairs[j].current_left == pairs[left].id {
                        let mut k_idx = 0;
                        for k in 0..pairs.len() {
                            if pairs[k].current_left == pairs[right].id {
                                k_idx = k;
                            }
                        }

                        pairs[k_idx].current_left = pairs[right].current_left;
                        pairs[j].current_left = pairs[right].id;
                        pairs[right].current_left = pairs[left].id;

                        pairs[right].moved = true;
                        let e = pairs.remove(right);
                        pairs.insert(j, e);

                        found_change = true;
                        break;
                    }
                }

                break;
            } else {
                left += 1;
            }
        }
        if !found_change {
            last_e -= 1;
        }
    }

    let mut expanded_list: Vec<i32> = vec![];
    expanded_list.extend(iter::repeat(pairs[0].id as i32).take(pairs[0].file_size));

    for i in 0..pairs.len() - 1 {
        for j in 0..pairs.len() {
            if pairs[j].current_left == pairs[i].id {
                expanded_list.extend(iter::repeat(pairs[j].id as i32).take(pairs[j].file_size));
                expanded_list.extend(iter::repeat(-1).take(pairs[j].free_space));
            }
        }
    }

    let checksum: i64 = expanded_list
        .iter()
        .enumerate()
        .filter(|&(_, &value)| value != FREE_SPACE)
        .map(|(position, &value)| position as i64 * value as i64)
        .sum();

    println!("Part2 checksum: {}", checksum);
}

fn main() -> io::Result<()> {
    let input = fs::read_to_string("input.txt")?;

    let values: Vec<usize> = input
        .chars()
        .filter_map(|x| x.to_digit(10))
        .map(|x| x as usize)
        .collect();

    solve_part1(&values);
    solve_part2(&values);

    Ok(())
}
