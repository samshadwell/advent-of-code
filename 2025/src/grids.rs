use anyhow::{Result, anyhow};

#[derive(Debug, PartialEq, Eq)]
pub struct Position {
    row: usize,
    col: usize,
}

#[derive(Debug)]
pub struct AdjacentIter<'a> {
    base: &'a Position,
    row_offset: i32,
    col_offset: i32,
}

impl Position {
    pub fn new(row: usize, col: usize) -> Self {
        Position { row, col }
    }

    pub fn adjacent<'a>(&'a self) -> AdjacentIter<'a> {
        AdjacentIter::new(self)
    }
}

impl<'a> AdjacentIter<'a> {
    fn new(base: &'a Position) -> Self {
        AdjacentIter {
            base,
            row_offset: -1,
            col_offset: -2,
        }
    }
}

impl<'a> Iterator for AdjacentIter<'a> {
    type Item = Position;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            self.col_offset += 1;
            if self.col_offset > 1 {
                self.col_offset = -1;
                self.row_offset += 1
            }

            if self.row_offset > 1 {
                return None;
            } else if self.col_offset == 0 && self.row_offset == 0 {
                // Skip the input coordinate itself
                continue;
            }

            let new_col = (self.base.col as i32) + self.col_offset;
            let new_row = (self.base.row as i32) + self.row_offset;

            if new_col >= 0 && new_row >= 0 {
                return Some(Position::new(new_row as usize, new_col as usize));
            }
        }
    }
}

pub struct Grid<T>(Vec<Vec<T>>);

impl<T> Grid<T> {
    pub fn new(g: Vec<Vec<T>>) -> Self {
        Grid(g)
    }

    pub fn get(&self, p: &Position) -> Option<&T> {
        self.0.get(p.row).and_then(|row| row.get(p.col))
    }

    pub fn set(&mut self, p: &Position, val: T) -> Result<()> {
        let cell = self.0.get_mut(p.row).and_then(|row| row.get_mut(p.col));

        match cell {
            None => Err(anyhow!("invalid position given to set")),
            Some(c) => {
                *c = val;
                Ok(())
            }
        }
    }

    pub fn all_positions(&self) -> PositionsIter<'_, T> {
        PositionsIter::new(self)
    }
}

pub struct PositionsIter<'a, T> {
    grid: &'a Grid<T>,
    curr_col: usize,
    curr_row: usize,
}

impl<'a, T> PositionsIter<'a, T> {
    fn new(grid: &'a Grid<T>) -> Self {
        PositionsIter {
            grid,
            curr_col: 0,
            curr_row: 0,
        }
    }
}

impl<'a, T> Iterator for PositionsIter<'a, T> {
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
    fn test_get() {
        let grid = Grid::new(vec![
            vec![0, 1, 2, 3], // force multi-line format
            vec![4, 5, 6, 7],
            vec![8, 9, 10, 11],
        ]);

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
        ]);

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
