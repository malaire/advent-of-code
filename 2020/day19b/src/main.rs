use regex::Regex;
use std::collections::{HashMap, HashSet};

static INPUT_A: &str = include_str!("input_a");
static INPUT_X: &str = include_str!("input");

// https://adventofcode.com/2020/day/19
// > (Remember, you only need to handle the rules you have;
// > building a solution that could handle any hypothetical
// > combination of rules would be significantly more difficult.)

fn main() {
    assert_eq!(solve(INPUT_A, 5), 12);
    assert_eq!(solve(INPUT_X, 8), 321);

    println!("{:?}", solve(INPUT_X, 8));
}

enum Rule {
    MatchA,
    MatchB,
    And(Vec<usize>),
    Or(Vec<usize>, Vec<usize>),
}
use Rule::*;

fn to_vec_usize(text: &str) -> Vec<usize> {
    text.split(" ")
        .map(|n| n.parse::<usize>().unwrap())
        .collect()
}

fn hashset(item: &str) -> HashSet<String> {
    let mut x = HashSet::new();
    x.insert(item.to_string());
    x
}

fn append(a: HashSet<String>, b: HashSet<String>) -> HashSet<String> {
    let mut x = HashSet::new();
    for item_a in a {
        for item_b in &b {
            x.insert(format!("{}{}", item_a, item_b));
        }
    }
    x
}

fn find_all(id: usize, rules: &HashMap<usize, Rule>) -> HashSet<String> {
    match rules.get(&id).unwrap() {
        MatchA => hashset("a"),
        MatchB => hashset("b"),
        And(sub_ids) => {
            let mut all = hashset("");
            for sub_id in sub_ids {
                all = append(all, find_all(*sub_id, rules));
            }
            all
        }
        Or(left_ids, right_ids) => {
            let mut all_left = hashset("");
            for sub_id in left_ids {
                all_left = append(all_left, find_all(*sub_id, rules));
            }

            let mut all_right = hashset("");
            for sub_id in right_ids {
                all_right = append(all_right, find_all(*sub_id, rules));
            }
            all_left.union(&all_right).cloned().collect::<HashSet<_>>()
        }
    }
}

fn solve(input: &str, sublen: usize) -> usize {
    let re_rule = Regex::new(r"^(\d+): (.+)$").unwrap();

    let mut rules: HashMap<usize, Rule> = HashMap::new();
    let mut all_31: HashSet<String> = HashSet::new();
    let mut all_42: HashSet<String> = HashSet::new();
    let mut matching_count = 0;

    let mut rules_input = true;
    for line in input.lines() {
        if line == "" {
            rules_input = false;
            all_31 = find_all(31, &rules);
            all_42 = find_all(42, &rules);

            for x in &all_31 {
                if x.len() != sublen {
                    panic!("unsupported input");
                }
            }
            for x in &all_42 {
                if x.len() != sublen {
                    panic!("unsupported input");
                }
            }
        } else if rules_input {
            let cap = re_rule.captures(line).unwrap();
            let id = cap[1].parse::<usize>().unwrap();
            let content = &cap[2];
            let rule = if content == "\"a\"" {
                MatchA
            } else if content == "\"b\"" {
                MatchB
            } else if content.contains('|') {
                let or: Vec<_> = content.split(" | ").collect();
                Or(to_vec_usize(or[0]), to_vec_usize(or[1]))
            } else {
                And(to_vec_usize(content))
            };
            rules.insert(id, rule);
        } else {
            if is_match(line, sublen, &all_31, &all_42) {
                matching_count += 1;
            }
        }
    }

    matching_count
}

//  0: 8 11
//  8: 42 | 42 42 | 42 42 42 | ...
// 11: 42 31 | 42 42 31 31 | 42 42 42 31 31 31 | ...
// ==> n times 42, then m times 31, n > m
fn is_match(text: &str, sublen: usize, all_31: &HashSet<String>, all_42: &HashSet<String>) -> bool {
    if text.len() % sublen != 0 {
        return false;
    }

    let mut parts = Vec::new();
    for n in 0..(text.len() / sublen) {
        parts.push(&text[n * sublen..(n + 1) * sublen]);
    }
    let part_count = parts.len();

    let mut max_leading_42s = 0;
    while max_leading_42s < part_count && all_42.contains(parts[max_leading_42s]) {
        max_leading_42s += 1;
    }

    let mut max_trailing_31s = 0;
    while max_trailing_31s < part_count && all_31.contains(parts[part_count - 1 - max_trailing_31s])
    {
        max_trailing_31s += 1;
    }

    max_leading_42s + max_trailing_31s >= part_count
        && max_leading_42s > max_trailing_31s
        && max_trailing_31s > 0
}
