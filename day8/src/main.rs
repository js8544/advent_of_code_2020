use std::collections::HashSet;

fn swap_jmp_nop(op: &mut Operation) {
    *op = match *op {
        Operation::Acc(n) => Operation::Acc(n),
        Operation::Jmp(n) => Operation::Nop(n),
        Operation::Nop(n) => Operation::Jmp(n),
    };
}

fn main() {
    let mut operations: Vec<Operation> =
        include_str!("input.txt").lines().map(parse_line).collect();
    for i in 0..operations.len() {
        swap_jmp_nop(operations.get_mut(i).unwrap());

        match run(&operations) {
            RunResult::Done(acc) => {
                println!("ans: {}", acc);
            }
            _ => (),
        }

        swap_jmp_nop(operations.get_mut(i).unwrap());
    }
}

enum Operation {
    Acc(i64),
    Jmp(i64),
    Nop(i64),
}

fn parse_line(line: &str) -> Operation {
    match &line[..3] {
        "acc" => Operation::Acc(line[4..].parse().unwrap()),
        "jmp" => Operation::Jmp(line[4..].parse().unwrap()),
        "nop" => Operation::Nop(line[4..].parse().unwrap()),
        _ => panic!("{} has invalid format!", line),
    }
}

#[derive(Debug)]
struct LoopResult {
    accumulator: i64,
    instruction: usize,
}

#[derive(Debug)]
enum RunResult {
    Done(i64),
    Loop(LoopResult),
}

fn run(operations: &Vec<Operation>) -> RunResult {
    let mut instruction = 0;
    let mut accumulator = 0;
    let mut instruction_run: HashSet<usize> = HashSet::new();
    loop {
        if instruction == operations.len() {
            return RunResult::Done(accumulator);
        } else {
            if instruction_run.contains(&instruction) {
                return RunResult::Loop(LoopResult {
                    accumulator,
                    instruction,
                });
            }
            instruction_run.insert(instruction);
            match operations.get(instruction).unwrap() {
                Operation::Acc(num) => {
                    accumulator += num;
                    instruction += 1;
                }
                Operation::Jmp(num) => {
                    instruction = ((instruction as i64) + num) as usize;
                }
                Operation::Nop(_) => {
                    instruction += 1;
                }
            };
        }
    }
}
