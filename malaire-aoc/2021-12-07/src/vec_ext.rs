//! Extension trait for `Vec`

use ndarray::{Array, Dimension, StrideShape};

// ======================================================================
// VecExt - PUBLIC

pub trait VecExt<'a, T: 'a> {
    /// Convert `Vec` into `Array` of given shape.
    ///
    /// ```rust
    /// use malaire_aoc::prelude::*;
    /// use ndarray::Array2;
    ///
    /// let a: Vec<usize> = vec![1, 2, 3, 4];
    /// let b: Array2<usize> = a.into_array((2, 2));
    /// assert_eq!(b, ndarray::arr2(&[[1, 2], [3, 4]]));
    /// ```
    fn into_array<Sh, D>(self, shape: Sh) -> Array<T, D>
    where
        Sh: Into<StrideShape<D>>,
        D: Dimension;

    /// Map `Vec` by-item, returning new `Vec`.
    /// - like `ArrayBase::map`
    ///
    /// ```rust
    /// use malaire_aoc::prelude::*;
    ///
    /// let a: Vec<usize> = vec![1,2,3];
    /// let b: Vec<bool> = a.map(|&x| x.is_power_of_two());
    /// assert_eq!(b, vec![true, true, false]);
    /// ```
    fn map<F, X>(&'a self, f: F) -> Vec<X>
    where
        F: FnMut(&'a T) -> X;
}

// ======================================================================
// VecExt - IMPL

impl<'a, T: 'a> VecExt<'a, T> for Vec<T> {
    fn into_array<Sh, D>(self, shape: Sh) -> Array<T, D>
    where
        Sh: Into<StrideShape<D>>,
        D: Dimension,
    {
        Array::from_shape_vec(shape, self).unwrap()
    }

    fn map<F, X>(&'a self, mut f: F) -> Vec<X>
    where
        F: FnMut(&'a T) -> X,
    {
        let mut new = Vec::with_capacity(self.len());
        for item in self.iter() {
            new.push(f(item));
        }
        new
    }
}
