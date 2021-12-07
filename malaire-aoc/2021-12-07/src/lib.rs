pub use crate::{array_base_ext::ArrayBaseExt, util::parse_numbers, vec_ext::VecExt};

mod array_base_ext;
pub mod prelude;
mod util;
mod vec_ext;

// ======================================================================
// FUNCTIONS

// Use `0` as `part` for test data and `1`/`2` for real data.
pub fn run<F, T>(part: usize, mut solve: F, input: &str, expected: T)
where
    F: FnMut(&str) -> T,
    T: std::cmp::PartialEq + std::fmt::Debug,
{
    let got = solve(input);
    if got != expected {
        println!();
        println!("GOT     : {:?}", got);
        println!("EXPECTED: {:?}", expected);
        println!();
        panic!();
    } else if part > 0 {
        println!("Part {}: {:?}", part, got);
    }
}
