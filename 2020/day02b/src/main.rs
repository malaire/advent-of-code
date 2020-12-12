use regex::Regex;

static INPUT_A: &str = "1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc";

static INPUT_B: &str = include_str!("input");

fn main() {
    assert_eq!(solve(INPUT_A), 1);
    assert_eq!(solve(INPUT_B), 321);

    println!("{:?}", solve(INPUT_B));
}

fn solve(input: &str) -> usize {
    // 1-3 b: cdefg
    let re = Regex::new(r"(?m)^(\d+)-(\d+) (.): (.+)$").unwrap();

    let mut valid_count: usize = 0;

    for x in re.captures_iter(input) {
        let pos_a: usize = x[1].parse().unwrap();
        let pos_b: usize = x[2].parse().unwrap();
        let needle: u8 = x[3].as_bytes()[0];
        let haystack: &[u8] = &x[4].as_bytes();

        if (haystack[pos_a - 1] == needle) ^ (haystack[pos_b - 1] == needle) {
            valid_count += 1;
        }
    }

    valid_count
}
