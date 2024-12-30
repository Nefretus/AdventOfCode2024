use regex::Regex;
use std::fs::File;
use std::io::{BufReader, Read};

fn solve(content: &str, re: &Regex, solve_part2: bool) -> i32 {
    let mut allow_mul = true;
    let mut counter: i32 = 0;

    for caps in re.captures_iter(&content) {
        if let Some(matched) = caps.get(0) {
            let text = matched.as_str();
            if solve_part2 {
                if text == "do()" {
                    allow_mul = true;
                } else if text == "don't()" {
                    allow_mul = false;
                }
            }
            if (!solve_part2 || allow_mul) && text.starts_with("mul(") {
                let x: i32 = caps.get(1).unwrap().as_str().parse().unwrap();
                let y: i32 = caps.get(2).unwrap().as_str().parse().unwrap();
                counter += x * y;
            }
        }
    }

    counter
}

fn main() -> std::io::Result<()> {
    let mut content = String::new();
    BufReader::new(File::open("input.txt")?).read_to_string(&mut content)?;
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)").unwrap();

    println!("Part1 solution: {}", solve(&content, &re, false));
    println!("Part2 solution: {}", solve(&content, &re, true));

    Ok(())
}
