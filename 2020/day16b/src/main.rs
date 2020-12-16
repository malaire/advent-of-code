use regex::Regex;
use std::collections::{HashMap, HashSet};

static INPUT_A: &str = "departure class: 0-1 or 4-19
departure row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9";

static INPUT_X: &str = include_str!("input");

fn main() {
    assert_eq!(solve(INPUT_A), 12 * 11);
    assert_eq!(solve(INPUT_X), 2843534243843);

    println!("{:?}", solve(INPUT_X));
}

#[derive(Debug)]
struct Field {
    name: String,
    min_a: usize,
    max_a: usize,
    min_b: usize,
    max_b: usize,
    potential_positions: HashSet<usize>,
}

fn solve(input: &str) -> usize {
    let re_rule = Regex::new(r"^([^:]+): (\d+)-(\d+) or (\d+)-(\d+)$").unwrap();

    let mut fields: Vec<Field> = Vec::new();
    let mut my_ticket: Vec<usize> = Vec::new();

    let mut next_is_my_ticket = false;
    for line in input.lines() {
        if re_rule.is_match(line) {
            let cap = re_rule.captures(line).unwrap();
            let name = cap[1].to_owned();
            let min_a: usize = cap[2].parse().unwrap();
            let max_a: usize = cap[3].parse().unwrap();
            let min_b: usize = cap[4].parse().unwrap();
            let max_b: usize = cap[5].parse().unwrap();
            fields.push(Field {
                name,
                min_a,
                max_a,
                min_b,
                max_b,
                potential_positions: HashSet::new(),
            });
        } else if line == "your ticket:" {
            next_is_my_ticket = true;
        } else if line == "nearby tickets:" || line == "" {
            // IGNORE
        } else {
            if next_is_my_ticket {
                my_ticket = line
                    .split(",")
                    .map(|n| n.parse::<usize>().unwrap())
                    .collect();

                let pos_count = my_ticket.len();

                for field in &mut fields {
                    for pos in 0..pos_count {
                        field.potential_positions.insert(pos);
                    }
                }

                next_is_my_ticket = false;
            } else {
                let mut discard = false;
                for n in line.split(",").map(|n| n.parse::<usize>().unwrap()) {
                    let mut can_be_valid = false;
                    for field in &fields {
                        if (n >= field.min_a && n <= field.max_a)
                            || (n >= field.min_b && n <= field.max_b)
                        {
                            can_be_valid = true;
                        }
                    }
                    if !can_be_valid {
                        discard = true;
                    }
                }

                if !discard {
                    for (pos, n) in line
                        .split(",")
                        .map(|n| n.parse::<usize>().unwrap())
                        .enumerate()
                    {
                        for field in fields.iter_mut() {
                            if n < field.min_a
                                || (n > field.max_a && n < field.min_b)
                                || (n > field.max_b)
                            {
                                field.potential_positions.remove(&pos);
                            }
                        }
                    }
                }
            }
        }
    }

    // println!("{:#?}", fields);

    let mut unsolved_fields = fields;
    let mut solved: HashMap<String, usize> = HashMap::new();

    while unsolved_fields.len() > 0 {
        let mut new_solved_pos: Vec<usize> = Vec::new();
        for field in &unsolved_fields {
            if field.potential_positions.len() == 1 {
                let pos = *field.potential_positions.iter().next().unwrap();
                solved.insert(field.name.clone(), pos);
                new_solved_pos.push(pos);
            }
        }

        unsolved_fields = unsolved_fields
            .into_iter()
            .filter(|f| f.potential_positions.len() != 1)
            .collect();

        for field in &mut unsolved_fields {
            for pos in &new_solved_pos {
                field.potential_positions.remove(&pos);
            }
        }
    }

    // println!("SOLVED\n{:#?}", solved);

    let mut mul = 1;
    for (name, pos) in &solved {
        if name.starts_with("departure ") {
            mul *= my_ticket[*pos];
        }
    }

    mul
}
