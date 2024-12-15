use std::fs;
use std::io;

const OBSTACLE: char = '#';
const BOX: char = 'O';
const ROBOT: char = '@';
const EMPTY: char = '.';

const BOX_PART1: char = '[';
const BOX_PART2: char = ']';

const UP: char = '^';
const DOWN: char = 'v';
const LEFT: char = '<';
const RIGHT: char = '>';

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}

fn display_grid(grid: &Vec<Vec<char>>) {
    for row in grid {
        println!("{}", row.iter().collect::<String>());
    }
    println!();
}

fn move_robot(grid: &mut Vec<Vec<char>>, curr_move: char, robot_pos: Point) -> Point {
    match curr_move {
        LEFT => {
            let mut next_y = robot_pos.y - 1;
            if grid[robot_pos.x][next_y] == EMPTY {
                grid[robot_pos.x][robot_pos.y] = EMPTY;
                grid[robot_pos.x][next_y] = ROBOT;
                Point {
                    x: robot_pos.x,
                    y: next_y,
                }
            } else if grid[robot_pos.x][next_y] == OBSTACLE {
                robot_pos
            } else {
                while grid[robot_pos.x][next_y] == BOX {
                    next_y -= 1;
                }
                if grid[robot_pos.x][next_y] == EMPTY {
                    while next_y != robot_pos.y {
                        grid[robot_pos.x][next_y] = grid[robot_pos.x][next_y + 1];
                        next_y += 1;
                    }
                    grid[robot_pos.x][robot_pos.y] = EMPTY;
                    return Point {
                        x: robot_pos.x,
                        y: robot_pos.y - 1,
                    };
                }
                robot_pos
            }
        }
        RIGHT => {
            let mut next_y = robot_pos.y + 1;
            if grid[robot_pos.x][next_y] == EMPTY {
                grid[robot_pos.x][robot_pos.y] = EMPTY;
                grid[robot_pos.x][next_y] = ROBOT;
                Point {
                    x: robot_pos.x,
                    y: next_y,
                }
            } else if grid[robot_pos.x][next_y] == OBSTACLE {
                robot_pos
            } else {
                while grid[robot_pos.x][next_y] == BOX {
                    next_y += 1;
                }
                if grid[robot_pos.x][next_y] == EMPTY {
                    while next_y != robot_pos.y {
                        grid[robot_pos.x][next_y] = grid[robot_pos.x][next_y - 1];
                        next_y -= 1;
                    }
                    grid[robot_pos.x][robot_pos.y] = EMPTY;
                    return Point {
                        x: robot_pos.x,
                        y: robot_pos.y + 1,
                    };
                }
                robot_pos
            }
        }
        UP => {
            let mut next_x = robot_pos.x - 1;
            if grid[next_x][robot_pos.y] == EMPTY {
                grid[robot_pos.x][robot_pos.y] = EMPTY;
                grid[next_x][robot_pos.y] = ROBOT;
                Point {
                    x: next_x,
                    y: robot_pos.y,
                }
            } else if grid[next_x][robot_pos.y] == OBSTACLE {
                robot_pos
            } else {
                while grid[next_x][robot_pos.y] == BOX {
                    next_x -= 1;
                }
                if grid[next_x][robot_pos.y] == EMPTY {
                    while next_x != robot_pos.x {
                        grid[next_x][robot_pos.y] = grid[next_x + 1][robot_pos.y];
                        next_x += 1;
                    }
                    grid[robot_pos.x][robot_pos.y] = EMPTY;
                    return Point {
                        x: robot_pos.x - 1,
                        y: robot_pos.y,
                    };
                }
                robot_pos
            }
        }
        DOWN => {
            let mut next_x = robot_pos.x + 1;
            if grid[next_x][robot_pos.y] == EMPTY {
                grid[robot_pos.x][robot_pos.y] = EMPTY;
                grid[next_x][robot_pos.y] = ROBOT;
                Point {
                    x: next_x,
                    y: robot_pos.y,
                }
            } else if grid[next_x][robot_pos.y] == OBSTACLE {
                robot_pos
            } else {
                while grid[next_x][robot_pos.y] == BOX {
                    next_x += 1;
                }
                if grid[next_x][robot_pos.y] == EMPTY {
                    while next_x != robot_pos.x {
                        grid[next_x][robot_pos.y] = grid[next_x - 1][robot_pos.y];
                        next_x -= 1;
                    }
                    grid[robot_pos.x][robot_pos.y] = EMPTY;
                    return Point {
                        x: robot_pos.x + 1,
                        y: robot_pos.y,
                    };
                }
                robot_pos
            }
        }
        _ => robot_pos,
    }
}

fn get_robot_pos(grid: &Vec<Vec<char>>) -> Option<Point> {
    for (i, row) in grid.iter().enumerate() {
        for (j, ch) in row.iter().enumerate() {
            if *ch == ROBOT {
                return Some(Point { x: i, y: j });
            }
        }
    }
    None
}

fn solve_part1(grid: &mut Vec<Vec<char>>, moves: &Vec<char>) {
    if let Some(mut robot_pos) = get_robot_pos(&grid) {
        for m in moves {
            robot_pos = move_robot(grid, *m, robot_pos);
        }
        display_grid(grid);
        let mut gps_score = 0;
        grid.iter().enumerate().for_each(|(i, row)| {
            row.iter().enumerate().for_each(|(j, _)| {
                if grid[i][j] == BOX {
                    gps_score += 100 * i + j;
                }
            });
        });
        println!("Sum of GPS coords: {}", gps_score);
    }
}

fn convert_grid(grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    grid.into_iter()
        .map(|row| {
            row.into_iter()
                .flat_map(|ch| match ch {
                    OBSTACLE => vec![OBSTACLE, OBSTACLE],
                    BOX => vec![BOX_PART1, BOX_PART2],
                    ROBOT => vec![ROBOT, EMPTY],
                    _ => vec![EMPTY, EMPTY],
                })
                .collect::<Vec<char>>()
        })
        .collect()
}

fn check_stacked_boxes(
    point: Point,
    grid: &Vec<Vec<char>>,
    boxes_to_move: &mut Vec<(Point, Point)>,
    direction: char,
) -> bool {
    if grid[point.x][point.y] == OBSTACLE {
        return false;
    }

    if grid[point.x][point.y] == EMPTY {
        return true;
    }

    if grid[point.x][point.y] == BOX_PART1 {
        boxes_to_move.push((
            Point {
                x: point.x,
                y: point.y,
            },
            Point {
                x: point.x,
                y: point.y + 1,
            },
        ));
    } else {
        boxes_to_move.push((
            Point {
                x: point.x,
                y: point.y - 1,
            },
            Point {
                x: point.x,
                y: point.y,
            },
        ));
    }

    let next_box1 = Point {
        x: if direction == UP {
            point.x - 1
        } else {
            point.x + 1
        },
        y: point.y,
    };
    let next_box2 = Point {
        x: if direction == UP {
            point.x - 1
        } else {
            point.x + 1
        },
        y: if grid[point.x][point.y] == BOX_PART1 {
            point.y + 1
        } else {
            point.y - 1
        },
    };

    return check_stacked_boxes(next_box1, grid, boxes_to_move, direction)
        && check_stacked_boxes(next_box2, grid, boxes_to_move, direction);
}

fn move_robot_part2(grid: &mut Vec<Vec<char>>, curr_move: char, robot_pos: Point) -> Point {
    match curr_move {
        LEFT => {
            let mut next_y = robot_pos.y - 1;
            if grid[robot_pos.x][next_y] == EMPTY {
                grid[robot_pos.x][robot_pos.y] = EMPTY;
                grid[robot_pos.x][next_y] = ROBOT;
                Point {
                    x: robot_pos.x,
                    y: next_y,
                }
            } else if grid[robot_pos.x][next_y] == OBSTACLE {
                robot_pos
            } else {
                // check the boxes to the left
                while grid[robot_pos.x][next_y] == BOX_PART2 {
                    next_y -= 2;
                }
                if grid[robot_pos.x][next_y] == EMPTY {
                    while next_y != robot_pos.y {
                        grid[robot_pos.x][next_y] = grid[robot_pos.x][next_y + 1];
                        next_y += 1;
                    }
                    grid[robot_pos.x][robot_pos.y] = EMPTY;
                    return Point {
                        x: robot_pos.x,
                        y: robot_pos.y - 1,
                    };
                }
                robot_pos
            }
        }
        RIGHT => {
            let mut next_y = robot_pos.y + 1;
            if grid[robot_pos.x][next_y] == EMPTY {
                grid[robot_pos.x][robot_pos.y] = EMPTY;
                grid[robot_pos.x][next_y] = ROBOT;
                Point {
                    x: robot_pos.x,
                    y: next_y,
                }
            } else if grid[robot_pos.x][next_y] == OBSTACLE {
                robot_pos
            } else {
                // check the boxes to the right
                while grid[robot_pos.x][next_y] == BOX_PART1 {
                    next_y += 2;
                }
                if grid[robot_pos.x][next_y] == EMPTY {
                    while next_y != robot_pos.y {
                        grid[robot_pos.x][next_y] = grid[robot_pos.x][next_y - 1];
                        next_y -= 1;
                    }
                    grid[robot_pos.x][robot_pos.y] = EMPTY;
                    return Point {
                        x: robot_pos.x,
                        y: robot_pos.y + 1,
                    };
                }
                robot_pos
            }
        }
        UP => {
            let next_x = robot_pos.x - 1;
            if grid[next_x][robot_pos.y] == EMPTY {
                grid[robot_pos.x][robot_pos.y] = EMPTY;
                grid[next_x][robot_pos.y] = ROBOT;
                Point {
                    x: next_x,
                    y: robot_pos.y,
                }
            } else if grid[next_x][robot_pos.y] == OBSTACLE {
                robot_pos
            } else {
                let possible_above1 = Point {
                    x: next_x - 1,
                    y: robot_pos.y,
                };
                let possible_above2 = Point {
                    x: next_x - 1,
                    y: if grid[next_x][robot_pos.y] == BOX_PART1 {
                        robot_pos.y + 1
                    } else {
                        robot_pos.y - 1
                    },
                };
                let mut boxes_to_move: Vec<(Point, Point)> = vec![];

                if grid[next_x][robot_pos.y] == BOX_PART1 {
                    boxes_to_move.push((
                        Point {
                            x: next_x,
                            y: robot_pos.y,
                        },
                        Point {
                            x: next_x,
                            y: robot_pos.y + 1,
                        },
                    ));
                } else {
                    boxes_to_move.push((
                        Point {
                            x: next_x,
                            y: robot_pos.y - 1,
                        },
                        Point {
                            x: next_x,
                            y: robot_pos.y,
                        },
                    ));
                }

                if check_stacked_boxes(possible_above1, grid, &mut boxes_to_move, curr_move)
                    && check_stacked_boxes(possible_above2, grid, &mut boxes_to_move, curr_move)
                {
                    boxes_to_move.sort_by(|a, b| a.0.x.cmp(&b.0.x));
                    for (box_part1, box_part2) in boxes_to_move {
                        grid[box_part1.x - 1][box_part1.y] = BOX_PART1;
                        grid[box_part2.x - 1][box_part2.y] = BOX_PART2;

                        grid[box_part1.x][box_part1.y] = EMPTY;
                        grid[box_part2.x][box_part2.y] = EMPTY;
                    }

                    grid[robot_pos.x][robot_pos.y] = EMPTY;
                    grid[robot_pos.x - 1][robot_pos.y] = ROBOT;

                    return Point {
                        x: robot_pos.x - 1,
                        y: robot_pos.y,
                    };
                }
                robot_pos
            }
        }
        DOWN => {
            let next_x = robot_pos.x + 1;
            if grid[next_x][robot_pos.y] == EMPTY {
                grid[robot_pos.x][robot_pos.y] = EMPTY;
                grid[next_x][robot_pos.y] = ROBOT;
                Point {
                    x: next_x,
                    y: robot_pos.y,
                }
            } else if grid[next_x][robot_pos.y] == OBSTACLE {
                robot_pos
            } else {
                let possible_above1 = Point {
                    x: next_x + 1,
                    y: robot_pos.y,
                };
                let possible_above2 = Point {
                    x: next_x + 1,
                    y: if grid[next_x][robot_pos.y] == BOX_PART1 {
                        robot_pos.y + 1
                    } else {
                        robot_pos.y - 1
                    },
                };

                let mut boxes_to_move: Vec<(Point, Point)> = vec![];

                if grid[next_x][robot_pos.y] == BOX_PART1 {
                    boxes_to_move.push((
                        Point {
                            x: next_x,
                            y: robot_pos.y,
                        },
                        Point {
                            x: next_x,
                            y: robot_pos.y + 1,
                        },
                    ));
                } else {
                    boxes_to_move.push((
                        Point {
                            x: next_x,
                            y: robot_pos.y - 1,
                        },
                        Point {
                            x: next_x,
                            y: robot_pos.y,
                        },
                    ));
                }

                if check_stacked_boxes(possible_above1, grid, &mut boxes_to_move, curr_move)
                    && check_stacked_boxes(possible_above2, grid, &mut boxes_to_move, curr_move)
                {
                    boxes_to_move.sort_by(|a, b| a.0.x.cmp(&b.0.x));

                    for (box_part1, box_part2) in boxes_to_move.iter().rev() {
                        grid[box_part1.x + 1][box_part1.y] = BOX_PART1;
                        grid[box_part2.x + 1][box_part2.y] = BOX_PART2;

                        grid[box_part1.x][box_part1.y] = EMPTY;
                        grid[box_part2.x][box_part2.y] = EMPTY;
                    }

                    grid[robot_pos.x][robot_pos.y] = EMPTY;
                    grid[robot_pos.x + 1][robot_pos.y] = ROBOT;

                    return Point {
                        x: robot_pos.x + 1,
                        y: robot_pos.y,
                    };
                }
                robot_pos
            }
        }
        _ => robot_pos,
    }
}

fn solve_part2(grid: &mut Vec<Vec<char>>, moves: &Vec<char>) {
    if let Some(mut robot_pos) = get_robot_pos(&grid) {
        for m in moves {
            robot_pos = move_robot_part2(grid, *m, robot_pos);
        }
        display_grid(grid);
        let mut gps_score = 0;
        grid.iter().enumerate().for_each(|(i, row)| {
            row.iter().enumerate().for_each(|(j, _)| {
                if grid[i][j] == BOX_PART1 {
                    gps_score += 100 * i + j;
                }
            });
        });
        println!("Part 1 sum of GPS coords: {}", gps_score);
    }
}

fn main() -> io::Result<()> {
    let input = fs::read_to_string("input.txt")?;

    let mut grid: Vec<Vec<char>> = input
        .lines()
        .take_while(|line| !line.trim().is_empty())
        .map(|line| line.trim().chars().collect())
        .collect();

    let moves: Vec<char> = input
        .lines()
        .skip_while(|line| !line.trim().is_empty())
        .flat_map(|line| line.trim().chars())
        .collect();

    let mut expanded_grid = convert_grid(grid.clone());

    solve_part1(&mut grid, &moves);
    solve_part2(&mut expanded_grid, &moves);

    Ok(())
}
