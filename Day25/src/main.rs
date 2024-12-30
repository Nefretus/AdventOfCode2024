fn main() -> std::io::Result<()> {
    let input = std::fs::read_to_string("input.txt")?;

    let mut it = input.lines().peekable();

    let mut locks: Vec<Vec<isize>> = vec![];
    let mut keys: Vec<Vec<isize>> = vec![];

    while it.peek().is_some() {
        if let Some(line) = it.next() {
            if line.trim().is_empty() {
                continue;
            }
            if line.chars().all(|c| c == '#') {
                let mut lock = vec![0; line.len()];
                while let Some(next_line) = it.peek() {
                    if next_line.trim().is_empty() {
                        break;
                    }
                    for (i, c) in next_line.chars().enumerate() {
                        if c == '#' {
                            lock[i] += 1;
                        }
                    }
                    it.next();
                }
                locks.push(lock);
            } else if line.chars().all(|c| c == '.') {
                let mut key = vec![-1; line.len()];
                let mut temp_stack = vec![];
                while let Some(next_line) = it.peek() {
                    if next_line.trim().is_empty() {
                        break;
                    }
                    temp_stack.push(next_line.to_string());
                    it.next();
                }
                for (line_nr, stacked_line) in temp_stack.iter().enumerate().rev() {
                    for (i, c) in stacked_line.chars().enumerate() {
                        if c == '#' {
                            key[i] += 1;
                        }
                    }
                }
                keys.push(key);
            }
        }
    }

    let mut count = 0;
    for lock in &locks {
        for key in &keys {
            if lock.iter().zip(key).all(|(&l, &k)| l + k <= 5) {
                count += 1;
            }
        }
    }

    println!("Total compatible lock/key pairs: {}", count);
    Ok(())
}
