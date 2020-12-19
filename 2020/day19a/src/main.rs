use regex::Regex;
use std::collections::{HashMap, HashSet};

static INPUT_A: &str = "0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: \"a\"
5: \"b\"

ababbb
bababa
abbbab
aaabbb
aaaabbb";

static INPUT_X: &str = include_str!("input");

fn main() {
    assert_eq!(solve(INPUT_A), 2);
    assert_eq!(solve(INPUT_X), 107);

    println!("{:?}", solve(INPUT_X));
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

fn solve(input: &str) -> usize {
    let re_rule = Regex::new(r"^(\d+): (.+)$").unwrap();

    let mut rules: HashMap<usize, Rule> = HashMap::new();
    let mut all: HashSet<String> = HashSet::new();
    let mut matching_count = 0;

    let mut rules_input = true;
    for line in input.lines() {
        if line == "" {
            rules_input = false;
            all = find_all(0, &rules);
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
            if all.contains(line) {
                matching_count += 1;
            }
        }
    }

    matching_count
}
