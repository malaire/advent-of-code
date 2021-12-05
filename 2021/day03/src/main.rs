static INPUT_A: &str =
    "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010";
static INPUT_X: &str = include_str!("input");

fn main() {
    assert_eq!(solve(INPUT_A), 198);
    assert_eq!(solve(INPUT_X), 3882564);
    println!("{:?}", solve(INPUT_X));

    assert_eq!(solve_2(INPUT_A), 230);
    assert_eq!(solve_2(INPUT_X), 3385170);
    println!("{:?}", solve_2(INPUT_X));
}

fn solve(input: &str) -> usize {
    let mut counts: [(usize, usize); 20] = [(0, 0); 20];

    let mut size = 0;
    for line in input.lines() {
        size = line.len();
        let mut pos = 0;
        for ch in line.chars() {
            match ch {
                '0' => counts[pos].0 += 1,
                '1' => counts[pos].1 += 1,
                _ => panic!(),
            }
            pos += 1;
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

fn filter(input: &[&str], find_oxygen: bool) -> usize {
    let mut input = input.to_vec();
    let mut filtered = Vec::new();

    let mut pos = 0;
    while input.len() > 1 {
        let mut count0 = 0;
        let mut count1 = 0;

        for line in input.iter() {
            if line.as_bytes()[pos] == b'0' {
                count0 += 1;
            } else {
                count1 += 1;
            }
        }

        for line in input.iter() {
            let byte = line.as_bytes()[pos];

            if find_oxygen {
                if (count0 > count1 && byte == b'0') || (count1 >= count0 && byte == b'1') {
                    filtered.push(line.clone());
                }
            } else {
                if (count0 <= count1 && byte == b'0') || (count1 < count0 && byte == b'1') {
                    filtered.push(line.clone());
                }
            }
        }

        input = filtered;
        filtered = Vec::new();
        pos += 1;
    }

    usize_from_bits_be(&input[0], b'1')
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
