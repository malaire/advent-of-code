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

static INPUT_B: &str = "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";

static INPUT_C: &str = include_str!("input");

fn main() {
    assert_eq!(solve(INPUT_A), 32);
    assert_eq!(solve(INPUT_B), 126);
    assert_eq!(solve(INPUT_C), 421550);

    println!("{:?}", solve(INPUT_C));
}

fn solve(input: &str) -> usize {
    let re_outer = Regex::new(r"(?m)^(\S+ \S+) bags contain (.+)\.$").unwrap();
    let re_inner = Regex::new(r"(\d+) (\S+ \S+) bags?").unwrap();

    let mut bags_solved: HashMap<_, usize> = HashMap::new();
    let mut bags_unsolved = HashMap::new();

    for line in input.lines() {
        let cap = re_outer.captures(line).unwrap();
        let outer = &cap[1];
        let contains = &cap[2];

        if contains == "no other" {
            bags_solved.insert(outer.to_owned(), 0);
        } else {
            let mut inners = Vec::new();
            for cap in re_inner.captures_iter(contains) {
                let count: usize = cap[1].parse().unwrap();
                let inner = &cap[2];
                inners.push((count, inner.to_owned()));
            }
            bags_unsolved.insert(outer.to_owned(), inners);
        }
    }

    while !bags_unsolved.is_empty() {
        for (outer, inners) in bags_unsolved.iter_mut() {
            let mut is_solved = true;
            let mut solved_count = 0;
            for (inner_count, inner) in inners.iter() {
                match bags_solved.get(inner) {
                    Some(inside_count) => solved_count += inner_count * (1 + inside_count),
                    None => is_solved = false,
                }
            }
            if is_solved {
                bags_solved.insert(outer.to_owned(), solved_count);
                *inners = Vec::new();
            }
        }
        bags_unsolved = bags_unsolved
            .into_iter()
            .filter(|(_, inners)| inners.len() != 0)
            .collect();
    }

    *bags_solved.get("shiny gold").unwrap()
}
