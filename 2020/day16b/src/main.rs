use regex::Regex;

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
    potential_positions_bitmap: u64,
}

impl Field {
    fn is_valid(&self, n: usize) -> bool {
        (n >= self.min_a && n <= self.max_a) || (n >= self.min_b && n <= self.max_b)
    }
}

fn solve(input: &str) -> usize {
    let re_rule = Regex::new(r"^([^:]+): (\d+)-(\d+) or (\d+)-(\d+)$").unwrap();

    let mut fields: Vec<Field> = Vec::new();
    let mut my_ticket: Vec<usize> = Vec::new();

    let mut next_is_my_ticket = true;
    for line in input.lines() {
        if let Some(cap) = re_rule.captures(line) {
            fields.push(Field {
                name: cap[1].to_owned(),
                min_a: cap[2].parse().unwrap(),
                max_a: cap[3].parse().unwrap(),
                min_b: cap[4].parse().unwrap(),
                max_b: cap[5].parse().unwrap(),
                potential_positions_bitmap: 0,
            });
        } else if line.contains(',') {
            let ticket: Vec<usize> = line.split(",").map(|n| n.parse().unwrap()).collect();
            if next_is_my_ticket {
                next_is_my_ticket = false;
                my_ticket = ticket;

                for field in &mut fields {
                    field.potential_positions_bitmap = (1 << my_ticket.len()) - 1;
                }
            } else {
                // ticket is valid if ALL numbers match ANY field
                if ticket.iter().all(|n| fields.iter().any(|f| f.is_valid(*n))) {
                    for (pos, n) in ticket.iter().enumerate() {
                        for field in fields.iter_mut() {
                            if !field.is_valid(*n) {
                                field.potential_positions_bitmap &= !(1 << pos);
                            }
                        }
                    }
                }
            }
        }
    }

    let mut unsolved_fields = fields;
    let mut unsolved_positions_bitmap: u64 = (1 << my_ticket.len()) - 1;
    let mut solved_product = 1;

    while unsolved_positions_bitmap != 0 {
        unsolved_fields.retain(|field| {
            if field.potential_positions_bitmap.count_ones() == 1 {
                let pos = field.potential_positions_bitmap.trailing_zeros() as usize;
                unsolved_positions_bitmap ^= 1 << pos;
                if field.name.starts_with("departure ") {
                    solved_product *= my_ticket[pos];
                }
                false
            } else {
                true
            }
        });

        for field in &mut unsolved_fields {
            field.potential_positions_bitmap &= unsolved_positions_bitmap;
        }
    }

    solved_product
}
