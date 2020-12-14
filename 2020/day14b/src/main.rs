use regex::Regex;
use std::collections::HashMap;

static INPUT_A: &str = "mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1";

static INPUT_X: &str = include_str!("input");

fn main() {
    assert_eq!(solve(INPUT_A), 208);
    assert_eq!(solve(INPUT_X), 3260587250457);

    println!("{:?}", solve(INPUT_X));
}

fn mem_write(address: u64, value: u64, floating: &[u64], mem: &mut HashMap<u64, u64>) {
    if floating.len() < 2 {
        mem.insert(address, value);
        if floating.len() == 1 {
            mem.insert(address ^ floating[0], value);
        }
    } else {
        mem_write(address, value, &floating[1..], mem);
        mem_write(address ^ floating[0], value, &floating[1..], mem);
    }
}

fn solve(input: &str) -> u64 {
    let re_mask = Regex::new(r"^mask = ([X01]{36})$").unwrap();
    let re_mem = Regex::new(r"^mem\[(\d+)\] = (\d+)$").unwrap();

    let mut mem: HashMap<u64, u64> = HashMap::new();

    let mut or_mask = 0;
    let mut floating: Vec<u64> = Vec::new();

    for line in input.lines() {
        if re_mask.is_match(line) {
            or_mask = 0;
            floating = Vec::new();
            let mask = &re_mask.captures(line).unwrap()[1];
            for (index, ch) in mask.chars().enumerate() {
                match ch {
                    'X' => floating.push(1 << (35 - index)),
                    '0' => (),
                    '1' => or_mask ^= 1 << (35 - index),
                    _ => panic!(""),
                }
            }
        } else {
            let cap = re_mem.captures(line).unwrap();
            let address: u64 = cap[1].parse().unwrap();
            let value = cap[2].parse().unwrap();
            mem_write(address | or_mask, value, &floating, &mut mem);
        }
    }

    let mut sum = 0;
    for value in mem.values() {
        sum += value;
    }

    sum
}
