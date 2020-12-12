use regex::Regex;
use std::collections::HashMap;

static INPUT_A: &str = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

static INPUT_B: &str = include_str!("input");

fn main() {
    assert_eq!(solve(INPUT_A), 4);
    assert_eq!(solve(INPUT_B), 233);

    println!("{:?}", solve(INPUT_B));
}

#[derive(Debug, PartialEq)]
enum StateSolved {
    NoGold,
    CanContainGold,
}
use StateSolved::*;

fn solve(input: &str) -> u64 {
    let re_outer = Regex::new(r"(?m)^(\S+ \S+) bags contain (.+)\.$").unwrap();
    let re_inner = Regex::new(r"(\d+) (\S+ \S+) bags?").unwrap();

    let mut bags_solved = HashMap::new();
    let mut bags_unsolved = HashMap::new();

    for line in input.lines() {
        let cap = re_outer.captures(line).unwrap();
        let outer = &cap[1];
        let contains = &cap[2];

        if contains == "no other" {
            bags_solved.insert(outer.to_owned(), NoGold);
        } else {
            let mut inners = Vec::new();
            let mut can_contain_gold = false;
            for cap in re_inner.captures_iter(contains) {
                let inner = &cap[2];
                if inner == "shiny gold" {
                    can_contain_gold = true;
                } else {
                    inners.push(inner.to_owned());
                }
            }
            if can_contain_gold {
                bags_solved.insert(outer.to_owned(), CanContainGold);
            } else {
                bags_unsolved.insert(outer.to_owned(), inners);
            }
        }
    }

    while !bags_unsolved.is_empty() {
        for (outer, inners) in bags_unsolved.iter_mut() {
            let mut unsolved_inners = Vec::new();
            let mut can_contain_gold = false;
            for inner in inners.iter() {
                match bags_solved.get(inner) {
                    Some(CanContainGold) => can_contain_gold = true,
                    Some(NoGold) => (),
                    None => unsolved_inners.push(inner.to_owned()),
                }
            }
            if can_contain_gold {
                bags_solved.insert(outer.to_owned(), CanContainGold);
                *inners = Vec::new();
            } else {
                if unsolved_inners.len() == 0 {
                    bags_solved.insert(outer.to_owned(), NoGold);
                }
                *inners = unsolved_inners;
            }
        }
        bags_unsolved = bags_unsolved
            .into_iter()
            .filter(|(_, inners)| inners.len() != 0)
            .collect();
    }

    let mut gold_count = 0;
    for (_, state) in bags_solved.iter() {
        if state == &CanContainGold {
            gold_count += 1;
        }
    }

    gold_count
}
