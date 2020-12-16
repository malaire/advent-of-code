use regex::Regex;

static INPUT_A: &str = "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12";

static INPUT_X: &str = include_str!("input");

fn main() {
    assert_eq!(solve(INPUT_A), 71);
    assert_eq!(solve(INPUT_X), 20060);

    println!("{:?}", solve(INPUT_X));
}

struct Rule {
    min_a: usize,
    max_a: usize,
    min_b: usize,
    max_b: usize,
}

fn solve(input: &str) -> usize {
    let re_rule = Regex::new(r"^[^:]+: (\d+)-(\d+) or (\d+)-(\d+)$").unwrap();

    let mut rules: Vec<Rule> = Vec::new();
    let mut invalid_sum = 0;

    let mut next_is_my_ticket = false;
    for line in input.lines() {
        if re_rule.is_match(line) {
            let cap = re_rule.captures(line).unwrap();
            let min_a: usize = cap[1].parse().unwrap();
            let max_a: usize = cap[2].parse().unwrap();
            let min_b: usize = cap[3].parse().unwrap();
            let max_b: usize = cap[4].parse().unwrap();
            rules.push(Rule {
                min_a,
                max_a,
                min_b,
                max_b,
            });
        } else if line == "your ticket:" {
            next_is_my_ticket = true;
        } else if line == "nearby tickets:" || line == "" {
            // IGNORE
        } else {
            for n in line.split(",").map(|n| n.parse::<usize>().unwrap()) {
                if next_is_my_ticket {
                    // IGNORE
                    next_is_my_ticket = false;
                } else {
                    let mut is_valid = false;
                    for rule in &rules {
                        if (n >= rule.min_a && n <= rule.max_a)
                            || (n >= rule.min_b && n <= rule.max_b)
                        {
                            is_valid = true;
                        }
                    }
                    if !is_valid {
                        invalid_sum += n;
                    }
                }
            }
        }
    }

    invalid_sum
}
