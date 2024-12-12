// pub enum TurnDir {
//     Right,
//     Left,
// }

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Dir {
    Up,
    Left,
    Down,
    Right,
}

impl Dir {
    // pub fn opposite(&self) -> Dir {
    //     match self {
    //         Dir::Up => Dir::Down,
    //         Dir::Right => Dir::Left,
    //         Dir::Down => Dir::Up,
    //         Dir::Left => Dir::Right,
    //     }
    // }
    pub fn clockwise(&self) -> Dir {
        match self {
            Dir::Up => Dir::Right,
            Dir::Right => Dir::Down,
            Dir::Down => Dir::Left,
            Dir::Left => Dir::Up,
        }
    }
    // pub fn counter_clockwise(&self) -> Dir {
    //     match self {
    //         Dir::Up => Dir::Left,
    //         Dir::Right => Dir::Up,
    //         Dir::Down => Dir::Right,
    //         Dir::Left => Dir::Down,
    //     }
    // }
    // pub fn turn(&self, turn: TurnDir) -> Dir {
    //     match turn {
    //         TurnDir::Right => self.clockwise(),
    //         TurnDir::Left => self.counter_clockwise(),
    //     }
    // }
}
