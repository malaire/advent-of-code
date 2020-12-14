use regex::Regex;
use std::collections::HashMap;

static INPUT_A: &str = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";

static INPUT_X: &str = include_str!("input");

fn main() {
    assert_eq!(solve(INPUT_A), 165);
    assert_eq!(solve(INPUT_X), 15403588588538);

    println!("{:?}", solve(INPUT_X));
}

fn solve(input: &str) -> u64 {
    let re_mask = Regex::new(r"^mask = ([X01]{36})$").unwrap();
    let re_mem = Regex::new(r"^mem\[(\d+)\] = (\d+)$").unwrap();

    let mut mem: HashMap<u64, u64> = HashMap::new();

    let mut and_mask = 0xFFFFFFFFF;
    let mut or_mask = 0;

    for line in input.lines() {
        if re_mask.is_match(line) {
            and_mask = 0xFFFFFFFFF;
            or_mask = 0;
            let mask = &re_mask.captures(line).unwrap()[1];
            for (index, ch) in mask.chars().enumerate() {
                match ch {
                    'X' => (),
                    '0' => and_mask ^= 1 << (35 - index),
                    '1' => or_mask ^= 1 << (35 - index),
                    _ => panic!(""),
                }
            }
        } else {
            let cap = re_mem.captures(line).unwrap();
            let address = cap[1].parse().unwrap();
            let mut value = cap[2].parse().unwrap();
            value = (value & and_mask) | or_mask;
            mem.insert(address, value);
        }
    }

    let mut sum = 0;
    for value in mem.values() {
        sum += value;
    }

    sum
}
