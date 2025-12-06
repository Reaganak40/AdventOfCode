use crate::solution_traits::{Solution, SolutionFactory};

mod day1;

/// Factory function to get solution for a given day
pub fn get_solution_for_day(inputs_dir : &str, day : u8) -> Box<dyn Solution> {
    match day {

        1 => day1::Day1::init(inputs_dir, day),
        _ => panic!("Failed to find solution for day: {}", day)
    }
}