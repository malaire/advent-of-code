use regex::{Captures, Regex};

static INPUT_A: &str = "1 + 2 * 3 + 4 * 5 + 6
1 + (2 * 3) + (4 * (5 + 6))
2 * 3 + (4 * 5)
5 + (8 * 3 + 9 + 3 * 4 * 3)";

static INPUT_B: &str = "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))";
static INPUT_C: &str = "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";

static INPUT_X: &str = include_str!("input");

fn main() {
    assert_eq!(solve(INPUT_A), 231 + 51 + 46 + 1445);
    assert_eq!(solve(INPUT_B), 669060);
    assert_eq!(solve(INPUT_C), 23340);
    assert_eq!(solve(INPUT_X), 216975281211165);

    println!("{:?}", solve(INPUT_X));
}

fn calculate(expr: &str) -> usize {
    let mut re_parentheses = Replacer::new(r"\(([^()]+)\)", |c| calculate(&c[1]));
    let mut re_add = Replacer::new(r"(\d+) \+ (\d+)", |c| c[1].to_usize() + c[2].to_usize());
    let mut re_mul = Replacer::new(r"(\d+) \* (\d+)", |c| c[1].to_usize() * c[2].to_usize());

    let mut expr = expr.to_string();
    while re_parentheses.replace(&mut expr)
        || re_add.replace(&mut expr)
        || re_mul.replace(&mut expr)
    {}
    expr.to_usize()
}

fn solve(input: &str) -> usize {
    let mut sum = 0;
    for line in input.lines() {
        sum += calculate(line);
    }
    sum
}

// ================================================================================
// StrExt

trait StrExt {
    fn to_usize(&self) -> usize;
}

impl StrExt for str {
    fn to_usize(&self) -> usize {
        self.parse::<usize>().unwrap()
    }
}

// ================================================================================
// Replacer

struct Replacer<R, S>
where
    R: FnMut(&Captures) -> S,
    S: ToString,
{
    regex: Regex,
    replacer: R,
}

impl<R, S> Replacer<R, S>
where
    R: FnMut(&Captures) -> S,
    S: ToString,
{
    fn new(regex: &str, replacer: R) -> Self {
        Replacer {
            regex: Regex::new(regex).unwrap(),
            replacer,
        }
    }

    /// Replace first match. Return `true` if match was found.
    fn replace(&mut self, text: &mut String) -> bool {
        let replacer = &mut self.replacer;
        match self
            .regex
            .replace(&text, |c: &Captures| replacer(c).to_string())
        {
            std::borrow::Cow::Borrowed(_) => false,
            std::borrow::Cow::Owned(new) => {
                *text = new;
                true
            }
        }
    }
}
