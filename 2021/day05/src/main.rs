use std::collections::HashMap;

use regex::Regex;

static INPUT_A: &str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";
static INPUT_X: &str = include_str!("input");

fn main() {
    assert_eq!(solve(INPUT_A), 5);
    let x = solve(INPUT_X);
    assert_eq!(x, 5373);
    println!("{:?}", x);

    assert_eq!(solve_2(INPUT_A), 12);
    let x = solve_2(INPUT_X);
    assert_eq!(x, 21514);
    println!("{:?}", x);
}

fn solve(input: &str) -> usize {
    let re = Regex::new(r"(?m)^(\d+),(\d+) -> (\d+),(\d+)$").unwrap();

    let mut counts: HashMap<(isize, isize), usize> = HashMap::new();

    for cap in re.captures_iter(input) {
        let x1: isize = cap[1].parse().unwrap();
        let y1: isize = cap[2].parse().unwrap();
        let x2: isize = cap[3].parse().unwrap();
        let y2: isize = cap[4].parse().unwrap();

        if x1 == x2 {
            let dy = if y1 < y2 { 1 } else { -1 };
            let mut y = y1;
            loop {
                *counts.entry((x1, y)).or_insert(0) += 1;
                if y == y2 {
                    break;
                }
                y += dy;
            }
        }

        if y1 == y2 {
            let dx = if x1 < x2 { 1 } else { -1 };
            let mut x = x1;
            loop {
                *counts.entry((x, y1)).or_insert(0) += 1;
                if x == x2 {
                    break;
                }
                x += dx;
            }
        }
    }

    counts.values().filter(|n| **n > 1).count()
}

fn solve_2(input: &str) -> usize {
    let re = Regex::new(r"(?m)^(\d+),(\d+) -> (\d+),(\d+)$").unwrap();

    let mut counts: HashMap<(isize, isize), usize> = HashMap::new();

    for cap in re.captures_iter(input) {
        let x1: isize = cap[1].parse().unwrap();
        let y1: isize = cap[2].parse().unwrap();
        let x2: isize = cap[3].parse().unwrap();
        let y2: isize = cap[4].parse().unwrap();

        let dx = if x1 < x2 {
            1
        } else if x1 > x2 {
            -1
        } else {
            0
        };

        let dy = if y1 < y2 {
            1
        } else if y1 > y2 {
            -1
        } else {
            0
        };

        let mut x = x1;
        let mut y = y1;

        loop {
            *counts.entry((x, y)).or_insert(0) += 1;
            if x == x2 && y == y2 {
                break;
            }
            x += dx;
            y += dy;
        }
    }

    counts.values().filter(|n| **n > 1).count()
}
