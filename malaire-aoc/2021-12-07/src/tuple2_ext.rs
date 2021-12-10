// ======================================================================
// Tuple2Ext - PUBLIC

pub trait Tuple2Ext
where
    Self: Sized,
{
    /// Returns up to 4 orthogonal neighbours in row-major order.
    fn neighbours_orthogonal(&self, array_dim: (usize, usize)) -> Vec<Self>;

    fn neighbours_orthogonal_iter(
        &self,
        array_dim: (usize, usize),
    ) -> std::vec::IntoIter<(usize, usize)>;
}

// ======================================================================
// Tuple2Ext - IMPL

impl Tuple2Ext for (usize, usize) {
    fn neighbours_orthogonal(&self, array_dim: (usize, usize)) -> Vec<Self> {
        let mut neighbours = Vec::with_capacity(4);

        if self.0 > 0 {
            neighbours.push((self.0 - 1, self.1));
        }

        if self.1 > 0 {
            neighbours.push((self.0, self.1 - 1));
        }

        if self.1 < array_dim.1 - 1 {
            neighbours.push((self.0, self.1 + 1));
        }

        if self.0 < array_dim.0 - 1 {
            neighbours.push((self.0 + 1, self.1));
        }

        neighbours
    }

    fn neighbours_orthogonal_iter(
        &self,
        array_dim: (usize, usize),
    ) -> std::vec::IntoIter<(usize, usize)> {
        self.neighbours_orthogonal(array_dim).into_iter()
    }
}

// ======================================================================
// TESTS

#[cfg(test)]
mod tests {
    use super::*;

    // ============================================================
    // neighbours_orthogonal

    #[test]
    fn neighbours_orthogonal_1x1() {
        assert_eq!((0, 0).neighbours_orthogonal((1, 1)), vec![]);
    }

    #[test]
    fn neighbours_orthogonal_corners() {
        assert_eq!((0, 0).neighbours_orthogonal((2, 2)), vec![(0, 1), (1, 0)]);
        assert_eq!((0, 1).neighbours_orthogonal((2, 2)), vec![(0, 0), (1, 1)]);
        assert_eq!((1, 0).neighbours_orthogonal((2, 2)), vec![(0, 0), (1, 1)]);
        assert_eq!((1, 1).neighbours_orthogonal((2, 2)), vec![(0, 1), (1, 0)]);
    }

    #[test]
    fn neighbours_orthogonal_sides() {
        assert_eq!(
            (0, 1).neighbours_orthogonal((3, 3)),
            vec![(0, 0), (0, 2), (1, 1)]
        );
        assert_eq!(
            (1, 0).neighbours_orthogonal((3, 3)),
            vec![(0, 0), (1, 1), (2, 0)]
        );
        assert_eq!(
            (1, 2).neighbours_orthogonal((3, 3)),
            vec![(0, 2), (1, 1), (2, 2)]
        );
        assert_eq!(
            (2, 1).neighbours_orthogonal((3, 3)),
            vec![(1, 1), (2, 0), (2, 2)]
        );
    }

    #[test]
    fn neighbours_orthogonal_middle() {
        assert_eq!(
            (1, 1).neighbours_orthogonal((3, 3)),
            vec![(0, 1), (1, 0), (1, 2), (2, 1)]
        );
    }
}
