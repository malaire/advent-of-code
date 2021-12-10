use std::{
    convert::From,
    default::Default,
    ops::{AddAssign, MulAssign},
};

use ndarray::{Array2, ArrayView};

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

/// Reads data into a byte array, one row per line.
///
/// Panics if lines have different lengths.
pub fn read_byte_array(data: &str) -> Array2<u8> {
    let col_count = data.lines().next().unwrap().as_bytes().len();
    let mut array = Array2::zeros((0, col_count));
    for line in data.lines() {
        let bytes: Vec<_> = line.bytes().collect();
        let row = ArrayView::from(&bytes);
        array
            .push_row(row)
            .expect("ERROR: Lines have different lengths");
    }
    array
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
