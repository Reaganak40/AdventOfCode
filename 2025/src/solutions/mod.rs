use crate::solution_traits::{Solution, SolutionFactory};

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;

/// Factory function to get solution for a given day
pub fn get_solution_for_day(inputs_dir: &str, day: u8) -> Option<Box<dyn Solution>> {
    match day {
        1 => Some(day01::Day1::init(inputs_dir, day)),
        2 => Some(day02::Day2::init(inputs_dir, day)),
        3 => Some(day03::Day3::init(inputs_dir, day)),
        4 => Some(day04::Day4::init(inputs_dir, day)),
        5 => Some(day05::Day5::init(inputs_dir, day)),
        6 => Some(day06::Day6::init(inputs_dir, day)),
        _ => None,
    }
}
