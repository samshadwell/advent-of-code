use anyhow::{Result, anyhow};

#[derive(Debug, PartialEq, Eq)]
pub struct Position {
    row: usize,
    col: usize,
}

impl Position {
    #[must_use]
    pub const fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }

    pub fn adjacent(&self) -> impl Iterator<Item = Self> + '_ {
        (-1..=1).flat_map(move |row_offset| {
            (-1..=1).filter_map(move |col_offset| {
                if row_offset == 0 && col_offset == 0 {
                    return None;
                }
                let new_row = self.row.checked_add_signed(row_offset as isize)?;
                let new_col = self.col.checked_add_signed(col_offset as isize)?;
                Some(Self::new(new_row, new_col))
            })
        })
    }
}

#[derive(Clone)]
pub struct Grid<T>(Vec<Vec<T>>);

impl<T> Grid<T> {
    /// # Errors
    /// If given vectors don't form a rectangular grid (they're jagged)
    pub fn new(g: Vec<Vec<T>>) -> Result<Self> {
        let first_row_len = g.first().map_or(0, std::vec::Vec::len);
        for row in &g {
            if row.len() != first_row_len {
                return Err(anyhow!("grid is not rectangular"));
            }
        }
        Ok(Self(g))
    }

    #[must_use]
    pub fn get(&self, p: &Position) -> Option<&T> {
        self.0.get(p.row).and_then(|row| row.get(p.col))
    }

    /// Set value of given position to given value
    /// # Errors
    /// If given position is not a valid position in grid
    pub fn set(&mut self, p: &Position, val: T) -> Result<()> {
        let cell = self.0.get_mut(p.row).and_then(|row| row.get_mut(p.col));

        cell.map_or_else(
            || Err(anyhow!("invalid position given to set")),
            |c| {
                *c = val;
                Ok(())
            },
        )
    }

    #[must_use]
    pub const fn all_positions(&self) -> PositionsIter<'_, T> {
        PositionsIter::new(self)
    }

    pub fn values(&self) -> impl Iterator<Item = &T> {
        self.0.iter().flatten()
    }

    #[must_use]
    pub const fn num_rows(&self) -> usize {
        self.0.len()
    }

    #[must_use]
    pub fn num_cols(&self) -> usize {
        self.0.first().map_or(0, std::vec::Vec::len)
    }
}

pub struct PositionsIter<'a, T> {
    grid: &'a Grid<T>,
    curr_col: usize,
    curr_row: usize,
}

impl<'a, T> PositionsIter<'a, T> {
    const fn new(grid: &'a Grid<T>) -> Self {
        PositionsIter {
            grid,
            curr_col: 0,
            curr_row: 0,
        }
    }
}

impl<T> Iterator for PositionsIter<'_, T> {
    type Item = Position;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let row = self.grid.0.get(self.curr_row)?;

            if self.curr_col >= row.len() {
                self.curr_col = 0;
                self.curr_row += 1;
                continue;
            }

            let result = Some(Position::new(self.curr_row, self.curr_col));
            self.curr_col += 1;
            return result;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::grids::{Grid, Position};

    #[test]
    fn test_new() {
        assert!(
            Grid::new(vec![
                vec![0, 1, 2], // force multi-line format
                vec![3, 4, 5],
            ])
            .is_ok()
        );
        assert!(Grid::<bool>::new(vec![]).is_ok());

        assert!(
            Grid::new(vec![
                vec![0, 1, 2], // force multi-line format
                vec![3]
            ])
            .is_err()
        );
    }

    #[test]
    fn test_get() {
        let grid = Grid::new(vec![
            vec![0, 1, 2, 3], // force multi-line format
            vec![4, 5, 6, 7],
            vec![8, 9, 10, 11],
        ])
        .expect("succeeds");

        // Valid
        assert_eq!(Some(&2), grid.get(&Position::new(0, 2)));
        assert_eq!(Some(&5), grid.get(&Position::new(1, 1)));

        // Out of bounds
        assert_eq!(None, grid.get(&Position::new(100, 100)))
    }

    #[test]
    fn test_adjacent_iter() {
        let grid = Grid::new(vec![
            vec![0, 1, 2, 3], // force multi-line format
            vec![4, 5, 6, 7],
            vec![8, 9, 10, 11],
        ])
        .expect("succeeds");

        assert_eq!(
            vec![Some(&1), Some(&4), Some(&5)],
            Position::new(0, 0)
                .adjacent()
                .map(|pos| grid.get(&pos))
                .collect::<Vec<_>>()
        );

        assert_eq!(
            vec![
                Some(&0),
                Some(&1),
                Some(&2),
                Some(&4),
                Some(&6),
                Some(&8),
                Some(&9),
                Some(&10)
            ],
            Position::new(1, 1)
                .adjacent()
                .map(|pos| grid.get(&pos))
                .collect::<Vec<_>>()
        );

        assert_eq!(
            vec![Some(&6), Some(&7), None, Some(&10), None, None, None, None],
            Position::new(2, 3)
                .adjacent()
                .map(|pos| grid.get(&pos))
                .collect::<Vec<_>>()
        )
    }
}
