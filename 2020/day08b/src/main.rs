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
    assert_eq!(solve(INPUT_A), 8);
    assert_eq!(solve(INPUT_B), 846);

    println!("{:?}", solve(INPUT_B));
}

#[derive(Copy, Clone)]
enum Instruction {
    Acc(i64),
    Jmp(i64),
    Nop(i64),
}
use Instruction::*;

fn solve(input: &str) -> i64 {
    let re = Regex::new(r"^(acc|jmp|nop) ([+-]\d+)$").unwrap();

    let mut code = Vec::new();

    for line in input.lines() {
        let cap = re.captures(line).unwrap();
        let command = &cap[1];
        let number = cap[2].parse().unwrap();
        code.push(match command {
            "acc" => Acc(number),
            "jmp" => Jmp(number),
            "nop" => Nop(number),
            _ => panic!(),
        });
    }

    let mut ip_changed: i64 = 0;
    loop {
        let mut accumulator = 0;
        let mut ip: i64 = 0;
        let mut visited = Vec::new();
        for _ in &code {
            visited.push(false);
        }

        while (ip < code.len() as i64) && (!visited[ip as usize]) {
            visited[ip as usize] = true;

            let mut instruction = code[ip as usize];

            if ip == ip_changed {
                instruction = match instruction {
                    Acc(n) => Acc(n),
                    Jmp(n) => Nop(n),
                    Nop(n) => Jmp(n),
                };
            }

            match instruction {
                Acc(n) => {
                    accumulator += n;
                    ip += 1;
                }
                Jmp(n) => ip += n,
                Nop(_) => ip += 1,
            }
        }
        if ip == code.len() as i64 {
            return accumulator;
        } else {
            ip_changed += 1;
        }
    }
}
