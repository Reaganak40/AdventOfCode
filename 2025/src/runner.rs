use crate::solutions::get_solution_for_day;

const NUM_CHALLENGES: u8 = 12;

/// Runner struct to manage execution of solutions
pub struct SolutionRunner {
    path_to_inputs: String,
}

impl SolutionRunner {
    pub fn new() -> Self {
        let path_to_inputs = std::env::var("ADVENT_INPUTS_DIR")
            .expect("Failed to get ADVENT_INPUTS_DIR from environment");
        Self { path_to_inputs }
    }

    #[allow(dead_code)]
    pub fn run_and_print(&self, day: u8) {
        let mut solution = get_solution_for_day(&self.path_to_inputs, day)
            .expect(&format!("Failed to get solution for day {}", day));

        println!("Day {}", day);
        println!("\tPart 1: {}", solution.part1());
        println!("\tPart 2: {}", solution.part2());
    }

    pub fn run_all_days(&self) {
        for day in 1..=NUM_CHALLENGES {
            let mut solution = match get_solution_for_day(&self.path_to_inputs, day) {
                Some(s) => s,
                None => continue,
            };

            println!("Day {}", day);
            println!("\tPart 1: {}", solution.part1());
            println!("\tPart 2: {}", solution.part2());
        }
    }
}
