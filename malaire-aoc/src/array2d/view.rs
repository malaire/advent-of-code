use std::ops::Index;

use super::{util, Array2D};

// ================================================================================
// TYPE

// https://en.wikipedia.org/wiki/Transformation_matrix#Affine_transformations
// ┌       ┐ ┌     ┐
// │ a b c │ │ row │
// │ d e f │ │ col │
// │ 0 0 1 │ │  1  │
// └       ┘ └     ┘

#[derive(Debug)]
pub struct View<'a, A> {
    array: &'a A,
    row_count: isize,
    col_count: isize,
    a: isize,
    b: isize,
    c: isize,
    d: isize,
    e: isize,
    f: isize,
}

// ================================================================================
// PRIVATE / SEMIPRIVATE

impl<'a, A: Array2D> View<'a, A> {
    fn convert(&self, row: usize, col: usize) -> (usize, usize) {
        let row_i = util::to_i(row);
        let col_i = util::to_i(col);

        let converted_row: isize = self.a * row_i + self.b * col_i + self.c;
        let converted_col: isize = self.d * row_i + self.e * col_i + self.f;

        assert!(converted_row >= 0 && converted_col >= 0);
        (converted_row as usize, converted_col as usize)
    }

    pub(crate) fn from_array(array: &'a A) -> Self {
        View {
            array,
            row_count: util::to_i(array.row_count()),
            col_count: util::to_i(array.col_count()),
            a: 1,
            b: 0,
            c: 0,
            d: 0,
            e: 1,
            f: 0,
        }
    }

    fn transform(mut self, a: isize, b: isize, c: isize, d: isize, e: isize, f: isize) -> Self {
        // abcdef is the new transformation to be added
        //
        // ┌       ┐ ┌       ┐     ┌                     ┐
        // │ A B C │ │ a b c │     │ Aa+Bd Ab+Be Ac+Bf+C │
        // │ D E F │ │ d e f │  =  │ Da+Ed Db+Ee Dc+Ef+F │
        // │ 0 0 1 │ │ 0 0 1 │     │     0     0       1 │
        // └       ┘ └       ┘     └                     ┘

        let new_a = self.a * a + self.b * d;
        let new_b = self.a * b + self.b * e;
        let new_c = self.a * c + self.b * f + self.c;
        let new_d = self.d * a + self.e * d;
        let new_e = self.d * b + self.e * e;
        let new_f = self.d * c + self.e * f + self.f;
        let new_row_count = (a * self.row_count + b * self.col_count).abs();
        let new_col_count = (d * self.row_count + e * self.col_count).abs();

        self.a = new_a;
        self.b = new_b;
        self.c = new_c;
        self.d = new_d;
        self.e = new_e;
        self.f = new_f;
        self.row_count = new_row_count;
        self.col_count = new_col_count;

        self
    }
}

// ================================================================================
// PUBLIC

impl<'a, A: Array2D> View<'a, A> {
    pub fn cw(self, degrees: usize) -> Self {
        assert!(degrees % 90 == 0);
        match (degrees / 90).rem_euclid(4) {
            0 => self,
            1 => {
                let row_count = self.row_count;
                self.transform(0, -1, row_count - 1, 1, 0, 0)
            }
            2 => unimplemented!(), // TODO
            3 => unimplemented!(), // TODO
            _ => unreachable!(),
        }
    }

    pub fn transpose(self) -> Self {
        self.transform(0, 1, 0, 1, 0, 0)
    }

    pub fn window(
        mut self,
        row_delta: usize,
        col_delta: usize,
        row_count: usize,
        col_count: usize,
    ) -> Self {
        self = self.transform(1, 0, util::to_i(row_delta), 0, 1, util::to_i(col_delta));
        self.row_count = util::to_i(row_count);
        self.col_count = util::to_i(col_count);
        self
    }
}

// ================================================================================
// impl Clone

impl<A> Clone for View<'_, A> {
    fn clone(&self) -> Self {
        Self {
            array: self.array,
            row_count: self.row_count,
            col_count: self.col_count,
            a: self.a,
            b: self.b,
            c: self.c,
            d: self.d,
            e: self.e,
            f: self.f,
        }
    }
}

// ================================================================================
// impl Index

impl<A: Array2D> Index<(usize, usize)> for View<'_, A> {
    type Output = A::Item;

    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        self.get(row, col).unwrap()
    }
}

// ================================================================================
// impl Array2D

impl<A: Array2D> Array2D for View<'_, A> {
    type Item = A::Item;

    unsafe fn get_unchecked(&self, row: usize, col: usize) -> &Self::Item {
        let (row, col) = self.convert(row, col);
        self.array.get_unchecked(row, col)
    }

    fn col_count(&self) -> usize {
        debug_assert!(self.col_count > 0);
        self.col_count as usize
    }

    fn row_count(&self) -> usize {
        debug_assert!(self.col_count > 0);
        self.row_count as usize
    }
}

// ================================================================================
// TESTS

#[cfg(test)]
mod test {
    use super::super::{Array2D, Vec2D, View};

    // ======================================================================
    // HELPERS

    trait ToTuple<T> {
        fn to_tuple(&self) -> (usize, usize, Vec<T>);
    }

    impl<A> ToTuple<A::Item> for View<'_, A>
    where
        A: Array2D,
        A::Item: Copy,
    {
        fn to_tuple(&self) -> (usize, usize, Vec<A::Item>) {
            (self.row_count(), self.col_count(), self.to_row_major())
        }
    }

    // ======================================================================
    // TESTS

    //  0  1  2
    //  3  4  5

    //  0  1  2  3  4
    //  5  6  7  8  9
    // 10 11 12 13 14
    // 15 16 17 18 19

    #[test]
    fn view() {
        let a = Vec2D::from_row_major(0..6, 2, 3);

        let v = a.view();
        assert_eq!(v.to_tuple(), (2, 3, vec![0, 1, 2, 3, 4, 5]));
    }

    #[test]
    fn cw90_transpose_cw90() {
        let a = Vec2D::from_row_major(0..6, 2, 3);

        //  3  0
        //  4  1
        //  5  2
        let v1 = a.view().cw(90);
        assert_eq!(v1.to_tuple(), (3, 2, vec![3, 0, 4, 1, 5, 2]));

        //  3  4  5
        //  0  1  2
        let v2 = v1.transpose();
        assert_eq!(v2.to_tuple(), (2, 3, vec![3, 4, 5, 0, 1, 2]));

        //  0  3
        //  1  4
        //  2  5
        let v3 = v2.cw(90);
        assert_eq!(v3.to_tuple(), (3, 2, vec![0, 3, 1, 4, 2, 5]));
    }

    #[test]
    fn transpose_transpose() {
        let a = Vec2D::from_row_major(0..6, 2, 3);

        let v1 = a.view().transpose();
        assert_eq!(v1.to_tuple(), (3, 2, vec![0, 3, 1, 4, 2, 5]));

        let v2 = v1.transpose();
        assert_eq!(v2.to_tuple(), (2, 3, vec![0, 1, 2, 3, 4, 5]));
    }

    // ================================================================================
    // window & cw90

    #[test]
    fn cw90_window_cw90() {
        let a = Vec2D::from_row_major(0..20, 4, 5);

        // 15 10  5  0
        // 16 11  6  1
        // 17 12  7  2
        // 18 13  8  3
        // 19 14  9  4
        let v1 = a.view().cw(90).window(1, 2, 3, 2);
        assert_eq!(v1.to_tuple(), (3, 2, vec![6, 1, 7, 2, 8, 3]));

        //  8  7  6
        //  3  2  1
        let v2 = v1.cw(90);
        assert_eq!(v2.to_tuple(), (2, 3, vec![8, 7, 6, 3, 2, 1]));
    }

    #[test]
    fn window_cw90_window() {
        let a = Vec2D::from_row_major(0..20, 4, 5);

        // 17 12  7
        // 18 13  8
        let v1 = a.view().window(1, 2, 3, 2).cw(90);
        assert_eq!(v1.to_tuple(), (2, 3, vec![17, 12, 7, 18, 13, 8]));

        let v2 = v1.window(1, 0, 1, 2);
        assert_eq!(v2.to_tuple(), (1, 2, vec![18, 13]));
    }

    // ================================================================================
    // window & transpose

    #[test]
    fn transpose_window() {
        let a = Vec2D::from_row_major(0..20, 4, 5);

        //  0  5 10 15
        //  1  6 11 16
        //  2  7 12 17
        //  3  8 13 18
        //  4  9 14 19
        let v1 = a.view().transpose().window(1, 2, 3, 2);
        assert_eq!(v1.to_tuple(), (3, 2, vec![11, 16, 12, 17, 13, 18]));
    }

    #[test]
    fn window_transpose_window() {
        let a = Vec2D::from_row_major(0..20, 4, 5);

        let v1 = a.view().window(1, 2, 3, 2);
        assert_eq!(v1.to_tuple(), (3, 2, vec![7, 8, 12, 13, 17, 18]));

        //  7 12 17
        //  8 13 18
        let v2 = v1.transpose();
        assert_eq!(v2.to_tuple(), (2, 3, vec![7, 12, 17, 8, 13, 18]));

        let v3 = v2.window(1, 0, 1, 2);
        assert_eq!(v3.to_tuple(), (1, 2, vec![8, 13]));
    }

    // ================================================================================
    // window & cw90 & transpose

    #[test]
    fn window_cw90_transpose_window() {
        let a = Vec2D::from_row_major(0..20, 4, 5);

        // 17 18
        // 12 13
        //  7  8
        let v1 = a.view().window(1, 2, 3, 2).cw(90).transpose();
        assert_eq!(v1.to_tuple(), (3, 2, vec![17, 18, 12, 13, 7, 8]));

        let v2 = v1.window(1, 0, 1, 2);
        assert_eq!(v2.to_tuple(), (1, 2, vec![12, 13]));
    }
}
