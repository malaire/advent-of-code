use std::ops::{Index, IndexMut};

use super::{Array2D, ArrayMut2D};

// ================================================================================
// TYPE

#[derive(Debug)]
pub struct Vec2D<T> {
    // row major order
    items: Vec<T>,
    row_count: usize,
    col_count: usize,
}

// ================================================================================
// PUBLIC

impl<T> Vec2D<T> {
    pub fn from_row_major<C>(collection: C, row_count: usize, col_count: usize) -> Self
    where
        C: IntoIterator<Item = T>,
    {
        let item_count = row_count * col_count;
        let items = collection.into_iter().take(item_count).collect::<Vec<T>>();
        if items.len() != item_count {
            panic!();
        }
        Vec2D {
            items,
            row_count,
            col_count,
        }
    }

    pub fn repeat_item(item: T, row_count: usize, col_count: usize) -> Self
    where
        T: Clone,
    {
        let item_count = row_count * col_count;
        let mut items = Vec::with_capacity(item_count);
        items.resize(item_count, item);
        Vec2D {
            items,
            row_count,
            col_count,
        }
    }
}

// ================================================================================
// impl Index

impl<T> Index<(usize, usize)> for Vec2D<T>
where
    Self: Array2D<Item = T>,
{
    type Output = T;

    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        self.get(row, col).unwrap()
    }
}

// ================================================================================
// impl IndexMut

impl<T> IndexMut<(usize, usize)> for Vec2D<T>
where
    Self: ArrayMut2D<Item = T>,
{
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut Self::Output {
        self.get_mut(row, col).unwrap()
    }
}

// ================================================================================
// impl Array2D

impl<T> Array2D for Vec2D<T> {
    type Item = T;

    unsafe fn get_unchecked(&self, row: usize, col: usize) -> &Self::Item {
        &self.items[row * self.col_count + col]
    }

    fn col_count(&self) -> usize {
        self.col_count
    }
    fn row_count(&self) -> usize {
        self.row_count
    }
}

// ================================================================================
// impl ArrayMut2D

impl<T> ArrayMut2D for Vec2D<T> {
    unsafe fn get_unchecked_mut(&mut self, row: usize, col: usize) -> &mut Self::Item {
        &mut self.items[row * self.col_count + col]
    }
}

// ================================================================================
// TESTS

#[cfg(test)]
mod test {
    use super::*;

    //  0  1  2
    //  3  4  5

    #[test]
    fn col_count() {
        let x = Vec2D::from_row_major(0..6, 2, 3);
        assert_eq!(x.col_count(), 3);
    }

    #[test]
    fn from_row_major() {
        let a = Vec2D::from_row_major(0..6, 2, 3);

        assert_eq!(a.row_count(), 2);
        assert_eq!(a.col_count(), 3);
        assert_eq!(a.to_row_major(), vec![0, 1, 2, 3, 4, 5]);
    }

    #[test]
    fn get() {
        let x = Vec2D::from_row_major(0..6, 2, 3);
        assert_eq!(x.get(0, 0), Some(&0));
        assert_eq!(x.get(1, 2), Some(&5));

        assert_eq!(x.get(0, 3), None);
        assert_eq!(x.get(2, 0), None);
    }

    #[test]
    fn row_count() {
        let x = Vec2D::from_row_major(0..6, 2, 3);
        assert_eq!(x.row_count(), 2);
    }
}
