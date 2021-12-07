use std::{
    convert::From,
    default::Default,
    ops::{AddAssign, MulAssign},
};

// ======================================================================
// FUNCTIONS

/// Parse sequences of ASCII digits as numbers, skip everything else.
pub fn parse_numbers<T>(data: &str) -> Vec<T>
where
    T: AddAssign<T> + MulAssign<T> + Default + From<u8>,
{
    let data = data.as_bytes();
    let mut numbers: Vec<T> = Vec::new();

    let mut pos = 0;
    while pos < data.len() {
        if data[pos].is_ascii_digit() {
            let mut number = Default::default();
            while pos < data.len() && data[pos].is_ascii_digit() {
                number *= 10.into();
                number += (data[pos] - b'0').into();
                pos += 1;
            }
            numbers.push(number);
        } else {
            pos += 1;
        }
    }

    numbers
}

// ======================================================================
// TESTS

#[cfg(test)]
mod tests {
    use super::*;

    // ============================================================
    // parse_numbers

    #[test]
    fn test_parse_numbers() {
        assert_eq!(parse_numbers::<usize>("xx123y4zz"), vec![123, 4]);
    }
}
