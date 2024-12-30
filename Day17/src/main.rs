use std::collections::HashMap;
use std::fs;
use std::io;

const REGISTER_A: u8 = 4;
const REGISTER_B: u8 = 5;
const REGISTER_C: u8 = 6;

const FORMAT_ERR_MSG: &str = "Incorrect input data";

#[derive(Debug, Clone)]
enum Instruction {
    Adv(u8),
    Bxl(u8),
    Bst(u8),
    Jnz(u8),
    Bxc(u8),
    Out(u8),
    Bdv(u8),
    Cdv(u8),
}

fn evaluate_combo_operand(operand: u8, registers: &HashMap<u8, u64>) -> u64 {
    match operand {
        0 => 0,
        1 => 1,
        2 => 2,
        3 => 3,
        4 => *registers.get(&REGISTER_A).unwrap(),
        5 => *registers.get(&REGISTER_B).unwrap(),
        6 => *registers.get(&REGISTER_C).unwrap(),
        _ => panic!("Invalid combo operand."),
    }
}

impl Instruction {
    fn evaluate(
        &self,
        registers: &mut HashMap<u8, u64>,
        output: &mut Vec<u64>,
        instruction_pointer: &mut usize,
    ) {
        match self {
            Instruction::Adv(operand) => {
                let divisor = 2u64.pow(evaluate_combo_operand(*operand, registers) as u32);
                if let Some(val) = registers.get_mut(&REGISTER_A) {
                    *val = *val / divisor;
                }
            }
            Instruction::Bxl(operand) => {
                if let Some(val) = registers.get_mut(&REGISTER_B) {
                    *val ^= *operand as u64;
                }
            }
            Instruction::Bst(operand) => {
                let value = evaluate_combo_operand(*operand, registers) % 8;
                if let Some(val) = registers.get_mut(&REGISTER_B) {
                    *val = value & (0b00000111);
                }
            }
            Instruction::Jnz(operand) => {
                if let Some(&val) = registers.get(&REGISTER_A) {
                    if val != 0 {
                        *instruction_pointer = *operand as usize;
                    }
                }
            }
            Instruction::Bxc(_) => {
                let val_c = *registers.get(&REGISTER_C).unwrap();
                if let Some(val_b) = registers.get_mut(&REGISTER_B) {
                    *val_b ^= val_c;
                }
            }
            Instruction::Out(operand) => {
                let value = evaluate_combo_operand(*operand, registers) % 8;
                output.push(value);
            }
            Instruction::Bdv(operand) => {
                let val_a = *registers.get(&REGISTER_A).unwrap();
                let divisor = 2u64.pow(evaluate_combo_operand(*operand, registers) as u32);
                if let Some(val) = registers.get_mut(&REGISTER_B) {
                    *val = val_a / divisor;
                }
            }
            Instruction::Cdv(operand) => {
                let val_a = *registers.get(&REGISTER_A).unwrap();
                let divisor = 2u64.pow(evaluate_combo_operand(*operand, registers) as u32);
                if let Some(val) = registers.get_mut(&REGISTER_C) {
                    *val = val_a / divisor;
                }
            }
        }
    }

    fn to_number(&self) -> u8 {
        match self {
            Instruction::Adv(_) => 0,
            Instruction::Bxl(_) => 1,
            Instruction::Bst(_) => 2,
            Instruction::Jnz(_) => 3,
            Instruction::Bxc(_) => 4,
            Instruction::Out(_) => 5,
            Instruction::Bdv(_) => 6,
            Instruction::Cdv(_) => 7,
        }
    }

    fn get_operand(&self) -> u8 {
        match self {
            Instruction::Adv(operand)
            | Instruction::Bxl(operand)
            | Instruction::Bst(operand)
            | Instruction::Jnz(operand)
            | Instruction::Bxc(operand)
            | Instruction::Out(operand)
            | Instruction::Bdv(operand)
            | Instruction::Cdv(operand) => *operand,
        }
    }
}

fn parse_registers(input: &str) -> HashMap<u8, u64> {
    input
        .lines()
        .take(3)
        .enumerate()
        .map(|(num, line)| {
            let key = num as u8 + 4;
            let value = line
                .split(':')
                .nth(1)
                .expect(FORMAT_ERR_MSG)
                .trim()
                .parse::<u64>()
                .expect(FORMAT_ERR_MSG);
            (key, value)
        })
        .collect()
}

fn parse_instructions(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .skip(4)
        .flat_map(|line| line.split(':').nth(1))
        .flat_map(|instructions| {
            instructions
                .trim()
                .split(',')
                .map(|x| x.parse::<u8>().expect("Failed to parse instruction"))
                .collect::<Vec<u8>>()
                .chunks(2)
                .map(|chunk| match chunk {
                    [opcode, operand] => match *opcode {
                        0 => Instruction::Adv(*operand),
                        1 => Instruction::Bxl(*operand),
                        2 => Instruction::Bst(*operand),
                        3 => Instruction::Jnz(*operand),
                        4 => Instruction::Bxc(*operand),
                        5 => Instruction::Out(*operand),
                        6 => Instruction::Bdv(*operand),
                        7 => Instruction::Cdv(*operand),
                        _ => panic!("Unknown opcode: {}", opcode),
                    },
                    _ => panic!("{}", FORMAT_ERR_MSG),
                })
                .collect::<Vec<Instruction>>()
        })
        .collect()
}

fn solve_part1(mut registers: HashMap<u8, u64>, program: &Vec<Instruction>) {
    let mut instruction_pointer: usize = 0;
    let mut output = Vec::new();

    while instruction_pointer < program.len() {
        let instruction = &program[instruction_pointer];
        let previous_pointer = instruction_pointer;

        instruction.evaluate(&mut registers, &mut output, &mut instruction_pointer);

        if previous_pointer == instruction_pointer {
            instruction_pointer += 1;
        }
    }

    println!(
        "Part1 program sequence: {}",
        output
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
            .join(",")
    );
}

fn solve_part2(program: &Vec<Instruction>) {
    let mut expected_output: Vec<u64> = program
        .iter()
        .flat_map(|instr| vec![instr.to_number() as u64, instr.get_operand() as u64])
        .collect();

    expected_output.reverse();

    println!(
        "Part2 register A value: {}",
        find_register_value(0, 0, &program, &expected_output)
    );
}

fn run_program(program: &Vec<Instruction>, register_a: u64) -> Vec<u64> {
    let mut registers: HashMap<u8, u64> =
        HashMap::from([(REGISTER_A, register_a), (REGISTER_B, 0), (REGISTER_C, 0)]);
    let mut output = Vec::new();
    let mut instruction_pointer = 0;
    while instruction_pointer < program.len() {
        let instruction = &program[instruction_pointer];
        let prev_pointer = instruction_pointer;

        instruction.evaluate(&mut registers, &mut output, &mut instruction_pointer);

        if prev_pointer == instruction_pointer {
            instruction_pointer += 1;
        }
    }
    return output;
}

fn find_register_value(
    register_val: u64,
    depth: usize,
    program: &Vec<Instruction>,
    expected_output: &Vec<u64>,
) -> u64 {
    if depth == expected_output.len() {
        return register_val;
    }

    for i in 0..8 {
        let output = run_program(program, register_val * (1 << 3) + i);
        if output.len() > 0 && output[0] == expected_output[depth] {
            let result =
                find_register_value(register_val * (1 << 3) + i, depth + 1, program, expected_output);
            if result != 0 {
                return result;
            }
        }
    }

    0
}

fn main() -> io::Result<()> {
    let input = fs::read_to_string("input.txt")?;

    let start_registers = parse_registers(&input);
    let program = parse_instructions(&input);

    solve_part1(start_registers, &program);
    solve_part2(&program);

    Ok(())
}
