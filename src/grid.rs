// TODO
// use std::fmt::Display;

use crate::point::Point;

#[derive(Debug, Clone)]
pub struct Grid<T> {
    content: Vec<T>,
    width: usize,
}

impl<T> Grid<T> {
    pub fn from_input<F>(input: &str, create_element: F) -> Self
    where
        F: Fn(char) -> T,
    {
        let content: Vec<T> = input
            .replace("\n", "")
            .chars()
            .map(create_element)
            .collect();
        let width = input.find('\n').unwrap();
        Grid::from_vec(content, width)
    }
    pub fn from_vec(content: Vec<T>, width: usize) -> Self {
        Self { content, width }
    }
    pub fn get(&self, p: &Point<usize>) -> &T {
        &self.content[p.x + p.y * self.width]
    }
    pub fn set(&mut self, p: &Point<usize>, value: T) {
        self.content[p.x + p.y * self.width] = value;
    }
    pub fn width(&self) -> usize {
        self.width
    }
    pub fn height(&self) -> usize {
        self.content.len() / self.width
    }
}

// impl<T> Display for Grid<T> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {}
// }
