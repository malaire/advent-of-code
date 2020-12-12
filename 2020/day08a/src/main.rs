use regex::Regex;

static INPUT_A: &str = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";

static INPUT_B: &str = include_str!("input");

fn main() {
    assert_eq!(solve(INPUT_A), 5);
    assert_eq!(solve(INPUT_B), 1723);

    println!("{:?}", solve(INPUT_B));
}

#[derive(Copy, Clone)]
enum Instruction {
    Acc(i64),
    Jmp(i64),
    Nop,
}
use Instruction::*;

fn solve(input: &str) -> i64 {
    let re = Regex::new(r"^(acc|jmp|nop) ([+-]\d+)$").unwrap();

    let mut code = Vec::new();

    for line in input.lines() {
        let cap = re.captures(line).unwrap();
        let command = &cap[1];
        let number = cap[2].parse().unwrap();
        match command {
            "acc" => code.push(Acc(number)),
            "jmp" => code.push(Jmp(number)),
            "nop" => code.push(Nop),
            _ => panic!(),
        }
    }

    let mut accumulator = 0;
    let mut ip: i64 = 0;
    let mut visited = Vec::new();
    for _ in &code {
        visited.push(false);
    }

    while !visited[ip as usize] {
        visited[ip as usize] = true;

        let instruction = code[ip as usize];
        match instruction {
            Acc(n) => {
                accumulator += n;
                ip += 1;
            }
            Jmp(n) => ip += n,
            Nop => ip += 1,
        }
    }
    accumulator
}
