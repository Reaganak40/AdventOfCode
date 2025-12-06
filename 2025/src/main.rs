mod runner;
mod solution_traits;
mod solutions;

use crate::runner::SolutionRunner;

fn main() {
    let runner = SolutionRunner::new();
    runner.run_and_print(1);
}