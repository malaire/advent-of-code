use malaire_aoc::run;

static INPUT_A: &str =
    "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010";
static INPUT_X: &str = include_str!("input");

fn main() {
    run(0, solve_1, INPUT_A, 198);
    run(1, solve_1, INPUT_X, 3882564);

    run(0, solve_2, INPUT_A, 230);
    run(2, solve_2, INPUT_X, 3385170);
}

fn solve_1(input: &str) -> usize {
    let size = input.lines().next().unwrap().len();

    let mut counts = vec![(0, 0); size];

    for line in input.lines() {
        for (pos, ch) in line.chars().enumerate() {
            match ch {
                '0' => counts[pos].0 += 1,
                '1' => counts[pos].1 += 1,
                _ => panic!(),
            }
        }
    }

    let gamma = usize_from_bits_fn_be(size, |n| counts[n].1 > counts[n].0);
    let epsilon = usize_from_bits_fn_be(size, |n| counts[n].1 < counts[n].0);

    gamma * epsilon
}

fn solve_2(input: &str) -> usize {
    let lines: Vec<_> = input.lines().collect();

    filter(&lines, true) * filter(&lines, false)
}

fn filter(numbers: &[&str], find_oxygen: bool) -> usize {
    let mut numbers = numbers.to_vec();

    let mut pos = 0;
    while numbers.len() > 1 {
        let mut split: [Vec<_>; 2] = [Vec::new(), Vec::new()];

        for number in numbers.iter() {
            let bit = if number.as_bytes()[pos] == b'0' { 0 } else { 1 };
            split[bit].push(number.clone());
        }

        let zero_is_most_common = split[0].len() > split[1].len();

        let wanted_bit = if find_oxygen ^ zero_is_most_common {
            1
        } else {
            0
        };

        numbers = split[wanted_bit].clone();
        pos += 1;
    }

    usize_from_bits_be(&numbers[0], b'1')
}

// ======================================================================
// UTIL

fn usize_from_bits_be<T: std::cmp::PartialEq, S: AsRef<[T]>>(bits: S, bit_one: T) -> usize {
    let mut x = 0;
    for bit in bits.as_ref() {
        x <<= 1;
        if *bit == bit_one {
            x += 1;
        }
    }
    x
}

fn usize_from_bits_fn_be<F: Fn(usize) -> bool>(count: usize, is_one: F) -> usize {
    let mut x = 0;
    for index in 0..count {
        x <<= 1;
        if is_one(index) {
            x += 1;
        }
    }
    x
}
