use itertools::Itertools;
use std::{
    collections::{HashMap, VecDeque},
    usize, vec,
};

const DIRECTIONS: [(isize, isize, char); 4] =
    [(-1, 0, '^'), (0, 1, '>'), (1, 0, 'v'), (0, -1, '<')];


fn bfs(keypad: &Vec<Vec<char>>, start: (usize, usize), end: char) -> Vec<Vec<char>> {
    let rows = keypad.len();
    let cols = keypad[0].len();
    let mut optimal: usize = usize::MAX;

    let mut paths: Vec<Vec<char>> = vec![];
    let mut queue: VecDeque<((usize, usize), Vec<char>)> = VecDeque::new();
    queue.push_back((start, vec![]));

    while let Some(current) = queue.pop_front() {
        let ((row, col), moves) = current;
        for &(dr, dc, dn) in DIRECTIONS.iter() {
            let new_row = row as isize + dr;
            let new_col = col as isize + dc;
            if new_row >= 0 && new_row < rows as isize && new_col >= 0 && new_col < cols as isize {
                let new_row = new_row as usize;
                let new_col = new_col as usize;
                if keypad[new_row][new_col] == 'X' {
                    continue;
                }
                if keypad[new_row][new_col] == end {
                    if moves.len() + 1 > optimal {
                        return paths;
                    }
                    optimal = moves.len() + 1;
                    let mut new_moves = moves.clone();
                    new_moves.push(dn);
                    new_moves.push('A');
                    paths.push(new_moves);
                } else {
                    let mut new_moves = moves.clone();
                    new_moves.push(dn);
                    queue.push_back(((new_row, new_col), new_moves));
                }
            }
        }
    }

    paths
}

fn calculate_combinations(keypad: Vec<Vec<char>>) -> HashMap<(char, char), Vec<Vec<char>>> {
    let positions = keypad
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter().enumerate().filter_map(move |(j, &val)| {
                if val != 'X' {
                    Some((val, (i, j)))
                } else {
                    None
                }
            })
        })
        .collect::<HashMap<_, _>>();

    let mut seqs: HashMap<(char, char), Vec<Vec<char>>> = HashMap::new();
    for &start in positions.keys() {
        for &dest in positions.keys() {
            if start == dest {
                seqs.insert((start, dest), vec![vec!['A']]);
                continue;
            }
            seqs.insert(
                (start, dest),
                bfs(&keypad, *positions.get(&start).unwrap(), dest),
            );
        }
    }

    seqs
}

fn convert_code_into_pairs(code: &Vec<char>) -> Vec<(char, char)> {
    let mut moves = Vec::new();
    if let Some(&first) = code.first() {
        moves.push(('A', first));
    }
    moves.extend(code.windows(2).map(|window| (window[0], window[1])));
    moves
}

fn get_solutions(code: &Vec<char>, seqs: &HashMap<(char, char), Vec<Vec<char>>>) -> Vec<Vec<char>> {
    let moves = convert_code_into_pairs(code);

    let options: Vec<Vec<Vec<char>>> = moves
        .iter()
        .filter_map(|(start, end)| seqs.get(&(*start, *end)).map(|seq| seq.clone()))
        .collect();

    options
        .into_iter()
        .multi_cartesian_product()
        .map(|x| x.into_iter().flatten().collect::<Vec<char>>())
        .collect()
}

fn compute<'a>(
    code: &'a Vec<char>,
    depth: usize,
    dir_seqs: &'a HashMap<(char, char), Vec<Vec<char>>>,
    dir_lengths: &HashMap<(char, char), usize>,
    cache: &mut HashMap<(&'a Vec<char>, usize), usize>,
) -> usize {
    if depth == 1 {
        return convert_code_into_pairs(code)
            .into_iter()
            .map(|x| dir_lengths.get(&x).unwrap_or(&0))
            .sum();
    }

    if let Some(&len) = cache.get(&(code, depth)) {
        return len;
    }

    let mut len = 0;
    for (start, end) in convert_code_into_pairs(&code) {
        len += dir_seqs
            .get(&(start, end))
            .unwrap()
            .iter()
            .map(|seq| compute(seq, depth - 1, dir_seqs, dir_lengths, cache))
            .min()
            .unwrap_or(0);
    }

    cache.insert((code, depth), len);
    len
}

fn solve(
    input: &str,
    num_robots: usize,
    dir_combinations: &HashMap<(char, char), Vec<Vec<char>>>,
    num_combinations: &HashMap<(char, char), Vec<Vec<char>>>,
) {
    let dir_lengths: HashMap<(char, char), usize> = dir_combinations
        .iter()
        .map(|(movement, paths)| (*movement, paths[0].len()))
        .collect();

    let mut total_complexity = 0;

    for line in input.lines() {
        let mut cache: HashMap<(&Vec<char>, usize), usize> = HashMap::new();
        let line_chr: Vec<char> = line.chars().collect();
        let min_solution = get_solutions(&line_chr, &num_combinations)
            .iter()
            .map(|sol| {
                compute(
                    &sol,
                    num_robots,
                    &dir_combinations,
                    &dir_lengths,
                    &mut cache,
                )
            })
            .min()
            .unwrap();
        total_complexity += min_solution * line[..line.len() - 1].parse::<usize>().unwrap();
    }

    println!("Solution for {} robots: {}", num_robots, total_complexity);
}

fn main() -> std::io::Result<()> {
    let input = std::fs::read_to_string("input.txt")?;

    let num_keypad = vec![
        vec!['7', '8', '9'],
        vec!['4', '5', '6'],
        vec!['1', '2', '3'],
        vec!['X', '0', 'A'],
    ];

    let dir_keypad: Vec<Vec<char>> = vec![
        vec!['X', '^', 'A'], 
        vec!['<', 'v', '>']
    ];

    let dir_combinations = calculate_combinations(dir_keypad);
    let num_combinations = calculate_combinations(num_keypad);

    solve(&input, 2, &dir_combinations, &num_combinations);
    solve(&input, 25, &dir_combinations, &num_combinations);

    Ok(())
}
