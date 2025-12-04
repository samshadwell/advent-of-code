use super::Position;

pub struct Grid<T>(Vec<Vec<T>>);

impl<T> Grid<T> {
    pub fn new(g: Vec<Vec<T>>) -> Self {
        Grid(g)
    }

    pub fn get(&self, p: Position) -> Option<T> {
        self.0.get(p.row)
        None
    }
}
