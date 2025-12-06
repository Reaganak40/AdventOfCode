use std::ffi::{OsStr, OsString};
use crate::solution_traits::SolutionFactory;

use crate::solutions::get_solution_for_day;

/// Runner struct to manage execution of solutions
pub struct SolutionRunner {

    path_to_inputs : String,

}

impl SolutionRunner {
    pub fn new() -> Self {
        let path_to_inputs = std::env::var("ADVENT_INPUTS_DIR").expect("Failed to get ADVENT_INPUTS_DIR from environment");
        Self { path_to_inputs }
    }

    pub fn run_and_print(&self, day : u8) {
        let mut solution = get_solution_for_day(&self.path_to_inputs, day);

        println!("Day {}", day);
        println!("\tPart 1: {}", solution.part1());
        println!("\tPart 2: {}", solution.part2());
    }
}