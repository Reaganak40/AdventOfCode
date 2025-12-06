use crate::solution_traits::{Solution, SolutionFactory, read_input};

pub struct Day1 {
    input : String,
}

impl Solution for Day1 {

    fn part1(&mut self) -> String {
        self.input.clone()
    }

    fn part2(&mut self) -> String {
        self.input.clone()
    }
}

impl SolutionFactory for Day1 {

    fn init(inputs_dir : &str, day : u8) -> Box<dyn Solution> {
        let input = read_input(inputs_dir, day);

        Box::new(Day1 {
            input : String::from_utf8_lossy(&input).to_string(),
        })
    }
}

