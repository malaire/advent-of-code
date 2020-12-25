use std::convert::TryFrom;

// ================================================================================
// SEMIPRIVATE HELPERS

pub(crate) fn to_i(x: usize) -> isize {
    isize::try_from(x).expect("row & col values must fit both `isize` and `usize`")
}
