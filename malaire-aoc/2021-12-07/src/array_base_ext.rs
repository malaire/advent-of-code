//! Extension trait for `ndarray::ArrayBase`

use ndarray::{iter::LanesIter, ArrayBase, Data, Dimension, RawData};
use std::iter::Chain;

// ======================================================================
// ArrayBaseExt - PUBLIC

pub trait ArrayBaseExt<S: RawData + Data, D: Dimension> {
    /// Iterate all columns.
    fn columns_iter(&self) -> LanesIter<'_, S::Elem, D::Smaller>;

    /// Iterate all rows.
    fn rows_iter(&self) -> LanesIter<'_, S::Elem, D::Smaller>;

    /// Iterate all rows and columns.
    fn rowcol_iter(
        &self,
    ) -> Chain<LanesIter<'_, S::Elem, D::Smaller>, LanesIter<'_, S::Elem, D::Smaller>> {
        self.rows_iter().chain(self.columns_iter())
    }
}

// ======================================================================
// ArrayBaseExt - IMPL

impl<S: RawData + ndarray::Data, D: Dimension> ArrayBaseExt<S, D> for ArrayBase<S, D> {
    fn columns_iter(&self) -> LanesIter<'_, S::Elem, D::Smaller> {
        self.columns().into_iter()
    }

    fn rows_iter(&self) -> LanesIter<'_, S::Elem, D::Smaller> {
        self.rows().into_iter()
    }
}
