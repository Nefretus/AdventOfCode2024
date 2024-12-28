use std::collections::HashMap;

struct Operation<'a> {
    wire1: &'a str,
    wire2: &'a str,
    operand: &'a str,
    output: &'a str,
}

fn evaluate_gate(input1: bool, input2: bool, operand: &str) -> bool {
    match operand {
        "AND" => input1 && input2,
        "OR" => input1 || input2,
        "XOR" => input1 ^ input2,
        _ => panic!("Unknown operation: {}", operand),
    }
}

fn to_u64(slice: &[u8]) -> u64 {
    slice.iter().rev().fold(0, |acc, &b| acc * 2 + b as u64)
}

fn resolve_circut(input: &str) -> HashMap<&str, bool> {
    let mut wires: HashMap<&str, bool> = input
        .lines()
        .take_while(|line| !line.trim().is_empty())
        .map(|line| {
            let mut parts = line.split(':');
            let key = parts.next().unwrap().trim();
            let value = match parts.next().unwrap().trim() {
                "1" => true,
                "0" => false,
                _ => panic!("Invalid value for boolean"),
            };
            (key, value)
        })
        .collect();

    let mut queue: Vec<Operation> = input
        .lines()
        .skip_while(|line| !line.trim().is_empty())
        .skip(1)
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let parts: Vec<&str> = line.split(" -> ").collect();
            let operands: Vec<&str> = parts[0].split_whitespace().collect();
            Operation {
                wire1: operands[0],
                wire2: operands[2],
                operand: operands[1],
                output: parts[1].trim(),
            }
        })
        .collect();

    while !queue.is_empty() {
        queue.retain(|op| {
            if let (Some(&input1), Some(&input2)) = (wires.get(op.wire1), wires.get(op.wire2)) {
                wires.insert(op.output, evaluate_gate(input1, input2, op.operand));
                false
            } else {
                true
            }
        });
    }

    wires
}

fn solve_part1(input: &str) {
    let wires: HashMap<&str, bool> = resolve_circut(input);

    let bits: Vec<u8> = (0..)
        .map(|count| format!("z{:02}", count))
        .map(|key| wires.get(key.as_str()).copied())
        .take_while(Option::is_some)
        .map(|wire| wire.unwrap() as u8)
        .collect();

    println!("Found decimal number: {:?}", to_u64(&bits));
}

fn format_wire(prefix: &str, num: u32) -> String {
    format!("{}{:02}", prefix, num)
}

fn check_z(operations: &HashMap<&str, (&str, &str, &str)>, wire: &str, num: u32) -> bool {
    if let Some((op, x, y)) = operations.get(wire) {
        if *op != "XOR" {
            return false;
        }
        if num == 0 {
            let mut wires = vec![x, y];
            wires.sort();
            return wires == [&format_wire("x", 0), &format_wire("y", 0)];
        }
        return (check_intermediate_xor(operations, x, num)
            && check_carry(operations, y, num))
            || (check_intermediate_xor(operations, y, num)
                && check_carry(operations, x, num));
    }
    false
}

fn check_intermediate_xor(
    operations: &HashMap<&str, (&str, &str, &str)>,
    wire: &str,
    num: u32,
) -> bool {
    if let Some((op, x, y)) = operations.get(wire) {
        if *op != "XOR" {
            return false;
        }
        let mut wires = vec![x, y];
        wires.sort();
        return wires == [&format_wire("x", num), &format_wire("y", num)];
    }
    false
}

fn check_carry(operations: &HashMap<&str, (&str, &str, &str)>, wire: &str, num: u32) -> bool {
    if let Some((op, x, y)) = operations.get(wire) {
        if num == 1 {
            if *op != "AND" {
                return false;
            }
            let mut wires = vec![x, y];
            wires.sort();
            return wires == [&format_wire("x", 0), &format_wire("y", 0)];
        }
        if *op != "OR" {
            return false;
        }
        return (check_direct_carry(operations, x, num - 1)
            && check_recarry(operations, y, num - 1))
            || (check_direct_carry(operations, y, num - 1)
                && check_recarry(operations, x, num - 1));
    }
    false
}

fn check_direct_carry(
    operations: &HashMap<&str, (&str, &str, &str)>,
    wire: &str,
    num: u32,
) -> bool {
    if let Some((op, x, y)) = operations.get(wire) {
        if *op != "AND" {
            return false;
        }
        let mut wires = vec![x, y];
        wires.sort();
        return wires == [&format_wire("x", num), &format_wire("y", num)];
    }
    false
}

fn check_recarry(operations: &HashMap<&str, (&str, &str, &str)>, wire: &str, num: u32) -> bool {
    if let Some((op, x, y)) = operations.get(wire) {
        if *op != "AND" {
            return false;
        }
        return (check_intermediate_xor(operations, x, num)
            && check_carry(operations, y, num))
            || (check_intermediate_xor(operations, y, num)
                && check_carry(operations, x, num));
    }
    false
}

fn progress(operations: &HashMap<&str, (&str, &str, &str)>) -> u32 {
    let mut i = 0;
    loop {
        if !check_z(operations, &format_wire("z", i), i) {
            break;
        }
        i += 1;
    }
    i
}

fn solve_part2(input: &str) {
    let mut operations: HashMap<&str, (&str, &str, &str)> = input
        .lines()
        .skip_while(|line| !line.trim().is_empty())
        .skip(1)
        .map(|line| {
            let parts: Vec<&str> = line.split(" -> ").collect();
            let operands: Vec<&str> = parts[0].split_whitespace().collect();
            (parts[1].trim(), (operands[1], operands[0], operands[2]))
        })
        .collect();

    let mut swaped_gates: Vec<(&str, &str)> = vec![];
    let keys: Vec<&str> = operations.keys().copied().collect();

    for _ in 0..4 {
        let baseline = progress(&operations);
        let mut stop = false;
        for i in 0..keys.len() {
            for j in 0..keys.len() {
                let x = keys[i];
                let y = keys[j];
                if x == y {
                    continue;
                }
                if let (Some(temp_x), Some(temp_y)) = (operations.remove(x), operations.remove(y)) {
                    operations.insert(x, temp_y);
                    operations.insert(y, temp_x);
                    if progress(&operations) > baseline {
                        swaped_gates.push((x, y));
                        stop = true;
                        break;
                    } else {
                        operations.insert(x, temp_x);
                        operations.insert(y, temp_y);
                    }
                }
            }
            if stop {
                break;
            }
        }
    }

    let mut flattened: Vec<&str> = swaped_gates
        .iter()
        .flat_map(|tup| std::iter::once(tup.0).chain(std::iter::once(tup.1)))
        .collect();

    flattened.sort();

    println!("Replaced gates: {}", flattened.join(","));
}

fn main() -> std::io::Result<()> {
    let input = std::fs::read_to_string("input.txt")?;

    solve_part1(&input);
    solve_part2(&input);

    Ok(())
}
