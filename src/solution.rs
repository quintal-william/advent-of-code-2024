pub trait Solution<T> {
    fn title(&self) -> &str;
    fn parse(input: &str) -> T;

    fn solve(&self) {
        println!("Title: {}", self.title());
        println!("Part 1: {}", self.solve_part_1());
        println!("Part 2: {}", self.solve_part_2());
    }
}

pub trait Year {}
