use crate::solution_traits::{Solution, SolutionFactory};

mod day01;
mod day02;

/// Factory function to get solution for a given day
pub fn get_solution_for_day(inputs_dir: &str, day: u8) -> Option<Box<dyn Solution>> {
    match day {
        1 => Some(day01::Day1::init(inputs_dir, day)),
        2 => Some(day02::Day2::init(inputs_dir, day)),
        _ => None,
    }
}
