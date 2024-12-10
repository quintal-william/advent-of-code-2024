use std::ops::{Add, Sub};

use num::Num;

use crate::dir::Dir;

#[derive(Debug, Hash, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point<T>
where
    T: Num + Copy,
{
    pub fn new(x: T, y: T) -> Self {
        Point { x, y }
    }
    pub fn dir_steps(&self, dir: &Dir, steps: T) -> Self {
        match dir {
            Dir::Up => Point::new(self.x, self.y - steps),
            Dir::Right => Point::new(self.x + steps, self.y),
            Dir::Down => Point::new(self.x, self.y + steps),
            Dir::Left => Point::new(self.x - steps, self.y),
        }
    }
    pub fn dir(&self, dir: &Dir) -> Self {
        self.dir_steps(dir, T::one())
    }
}

impl<T: Add<Output = T>> Add for Point<T>
where
    T: Num + Copy,
{
    type Output = Point<T>;
    fn add(self, rhs: Self) -> Self::Output {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<T: Sub<Output = T>> Sub for Point<T>
where
    T: Num + Copy,
{
    type Output = Point<T>;
    fn sub(self, rhs: Self) -> Self::Output {
        Point::new(self.x - rhs.x, self.y - rhs.y)
    }
}
