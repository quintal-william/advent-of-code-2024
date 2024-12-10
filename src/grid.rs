use crate::point::Point;

pub struct Grid<T> {
    content: Vec<T>,
    width: usize,
}

impl<T> Grid<T> {
    pub fn from_vec(content: Vec<T>, width: usize) -> Self {
        Self { content, width }
    }
    pub fn get<U>(&self, p: &Point<usize>) -> &T {
        &self.content[p.x + p.y * self.width]
    }
    pub fn get_mut(&mut self, loc: &Point<usize>) -> &mut T {
        &mut self.content[loc.x + loc.y * self.width]
    }
    pub fn width(&self) -> usize {
        self.width
    }
    pub fn height(&self) -> usize {
        self.content.len() / self.width
    }
}
