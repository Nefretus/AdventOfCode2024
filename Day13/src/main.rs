use std::collections::HashMap;
use std::{cmp, fs};
use std::io;

struct ButtonDesc {
    x_step: isize,
    y_step: isize,
    cost: isize,
}

struct Goal {
    x: isize,
    y: isize,
}

fn solve(
    x: isize,
    y: isize,
    button_a: &ButtonDesc,
    button_b: &ButtonDesc,
    goal: &Goal,
    memo: &mut HashMap<(isize, isize), isize>,
) -> isize {
    if x == goal.x && y == goal.y {
        return 0; 
    }
    if x > goal.x || y > goal.y {
        return isize::MAX; 
    }
    if let Some(&cost) = memo.get(&(x, y)) {
        return cost;
    }

    let cost_a = solve(
        x + button_a.x_step,
        y + button_a.y_step,
        button_a,
        button_b,
        goal,
        memo,
    ).saturating_add(button_a.cost);

    let cost_b = solve(
        x + button_b.x_step,
        y + button_b.y_step,
        button_a,
        button_b,
        goal,
        memo,
    ).saturating_add(button_b.cost);

    let result = cmp::min(cost_a, cost_b);
    memo.insert((x, y), result);
    result
}

fn solve_math(button_a: &ButtonDesc, button_b: &ButtonDesc, goal: &Goal) -> Option<isize> {
    let den = button_a.x_step * button_b.y_step - button_a.y_step * button_b.x_step;
    if den == 0 {
        return None; 
    }

    let num_a = goal.x * button_b.y_step - goal.y * button_b.x_step;
    if num_a % den != 0 {
        return None;
    }
    let a_clicks = num_a / den;

    let num_b = goal.x - button_a.x_step * a_clicks;
    if num_b % button_b.x_step != 0 {
        return None; 
    }
    let b_clicks = num_b / button_b.x_step;
    
    Some(a_clicks * button_a.cost + b_clicks * button_b.cost)
}

fn parse_input(input: &str, solve_part2: bool) -> Vec<(ButtonDesc, ButtonDesc, Goal)> {
    let mut machines = Vec::new();
    let mut lines = input.lines().filter(|x| !x.trim().is_empty());

    while let (Some(button_a_line), Some(button_b_line), Some(prize_line)) =
        (lines.next(), lines.next(), lines.next())
    {
        let (a_x, a_y) = parse_button_line(button_a_line);
        let (b_x, b_y) = parse_button_line(button_b_line);
        let (goal_x, goal_y) = parse_prize_line(prize_line, solve_part2);

        let button_a = ButtonDesc {
            x_step: a_x,
            y_step: a_y,
            cost: 3,
        };
        
        let button_b = ButtonDesc {
            x_step: b_x,
            y_step: b_y,
            cost: 1,
        };
        
        let goal = Goal {
            x: goal_x,
            y: goal_y,
        };

        machines.push((button_a, button_b, goal));
    }

    machines
}

fn parse_button_line(line: &str) -> (isize, isize) {
    let parts: Vec<&str> = line.split_whitespace().collect();

    let x_step: isize = parts[2]
        .chars()
        .filter(|x| x.is_digit(10))
        .collect::<String>()
        .parse()
        .unwrap();
        
    let y_step: isize = parts[3]
        .chars()
        .filter(|y| y.is_digit(10))
        .collect::<String>()
        .parse()
        .unwrap();
        
    (x_step, y_step)
}

fn parse_prize_line(line: &str, solve_part2: bool) -> (isize, isize) {
    let overflow = if !solve_part2 {0} else { 10000000000000 };
    let parts: Vec<&str> = line.split_whitespace().collect();
    let x: isize = parts[1][2..].trim_end_matches(',').parse().unwrap();
    let y: isize = parts[2][2..].parse().unwrap();
    (x + overflow, y + overflow)
}


fn main() -> io::Result<()>{
    let input = fs::read_to_string("input.txt")?;
    
    {
        println!("Running brute-force approach");
        let machines = parse_input(&input, false);
        let mut total_tokens = 0;
        for (i, (button_a, button_b, goal)) in machines.iter().enumerate() {
            let mut memo = HashMap::new();
            let tokens = solve(0, 0, button_a, button_b, goal, &mut memo);
            if tokens != isize::MAX {
                total_tokens += tokens;
            }
        }
        println!("Part1: total tokens {}", total_tokens);
    }
    
    {
        println!("Running linear equation approach");
        let machines = parse_input(&input, true);
        let mut total_tokens = 0;
        for (i, (button_a, button_b, goal)) in machines.iter().enumerate() {
            if let Some(tokens) = solve_math(&button_a, &button_b, &goal) {
                total_tokens += tokens;
            }
        }
        println!("Part2: total tokens {}", total_tokens);
    }

    Ok(())
}
