static INPUT_A: &str = "abc\n\na\nb\nc\n\nab\nac\n\na\na\na\na\n\nb";

static INPUT_B: &str = include_str!("input");

fn main() {
    assert_eq!(solve(INPUT_A), 11);
    assert_eq!(solve(INPUT_B), 6351);

    println!("{:?}", solve(INPUT_B));
}

fn solve(input: &str) -> usize {
    let mut sum = 0;
    for group in input.split("\n\n") {
        let mut seen = vec![false; 26];
        let mut count = 0;

        for chr in group.chars() {
            if chr >= 'a' && chr <= 'z' {
                if !seen[chr as usize - ('a' as usize)] {
                    seen[chr as usize - ('a' as usize)] = true;
                    count += 1;
                }
            }
        }
        sum += count;
    }
    sum
}
