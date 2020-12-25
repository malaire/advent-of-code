mod util;
mod vec;
mod view;
pub use self::{vec::Vec2D, view::View};

// ================================================================================
// Array2D

pub trait Array2D: Sized {
    type Item;

    fn col(&self, col: usize) -> DeltaIterator<'_, Self> {
        DeltaIterator::new(self, 0, util::to_i(col), 1, 0)
    }

    fn col_count(&self) -> usize;

    fn get(&self, row: usize, col: usize) -> Option<&Self::Item> {
        if row < self.row_count() && col < self.col_count() {
            Some(unsafe { self.get_unchecked(row, col) })
        } else {
            None
        }
    }

    // - `row < row_count()`
    // - `col < col_count()`
    unsafe fn get_unchecked(&self, row: usize, col: usize) -> &Self::Item;

    fn row(&self, row: usize) -> DeltaIterator<'_, Self> {
        DeltaIterator::new(self, util::to_i(row), 0, 0, 1)
    }

    fn row_count(&self) -> usize;

    fn row_major(&self) -> RowMajor<'_, Self> {
        RowMajor::new(self)
    }

    fn to_row_major(&self) -> Vec<Self::Item>
    where
        Self::Item: Copy,
    {
        let row_count = self.row_count();
        let col_count = self.col_count();

        let mut items = Vec::with_capacity(row_count * col_count);
        for row in 0..row_count {
            for col in 0..col_count {
                items.push(unsafe { *self.get_unchecked(row, col) });
            }
        }
        items
    }

    fn view(&self) -> View<Self> {
        View::from_array(&self)
    }
}

// ================================================================================
// ArrayMut2D

pub trait ArrayMut2D: Array2D {
    fn get_mut(&mut self, row: usize, col: usize) -> Option<&mut Self::Item> {
        if row < self.row_count() && col < self.col_count() {
            Some(unsafe { self.get_unchecked_mut(row, col) })
        } else {
            None
        }
    }

    // - `row < row_count()`
    // - `col < col_count()`
    unsafe fn get_unchecked_mut(&mut self, row: usize, col: usize) -> &mut Self::Item;
}

// ================================================================================
// DeltaIterator

pub struct DeltaIterator<'a, A: Array2D> {
    array: &'a A,
    next_row: isize,
    next_col: isize,
    row_delta: isize,
    col_delta: isize,
    ended: bool,
}

impl<'a, A: Array2D> DeltaIterator<'a, A> {
    fn new(
        array: &'a A,
        row_first: isize,
        col_first: isize,
        row_delta: isize,
        col_delta: isize,
    ) -> Self {
        Self {
            array,
            next_row: row_first,
            next_col: col_first,
            row_delta,
            col_delta,
            ended: false,
        }
    }
}

impl<'a, A: Array2D> Iterator for DeltaIterator<'a, A> {
    type Item = &'a A::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if self.ended {
            None
        } else {
            debug_assert!(self.next_row >= 0 && self.next_col >= 0);
            match self
                .array
                .get(self.next_row as usize, self.next_col as usize)
            {
                Some(item) => {
                    self.next_row += self.row_delta;
                    self.next_col += self.col_delta;
                    if self.next_row < 0 || self.next_col < 0 {
                        self.ended = true;
                    }
                    Some(item)
                }
                None => {
                    self.ended = true;
                    None
                }
            }
        }
    }
}

// ================================================================================
// RowMajor

pub struct RowMajor<'a, A: Array2D> {
    array: &'a A,
    next_row: usize,
    next_col: usize,
    ended: bool,
}

impl<'a, A: Array2D> RowMajor<'a, A> {
    fn new(array: &'a A) -> Self {
        Self {
            array,
            next_row: 0,
            next_col: 0,
            ended: false,
        }
    }
}

impl<'a, A: Array2D> Iterator for RowMajor<'a, A> {
    type Item = &'a A::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if self.ended {
            None
        } else {
            match self.array.get(self.next_row, self.next_col) {
                Some(item) => {
                    if self.next_col < self.array.col_count() - 1 {
                        self.next_col += 1;
                    } else if self.next_row < self.array.row_count() - 1 {
                        self.next_row += 1;
                        self.next_col = 0;
                    } else {
                        self.ended = true;
                    }
                    Some(item)
                }
                None => {
                    self.ended = true;
                    None
                }
            }
        }
    }
}
