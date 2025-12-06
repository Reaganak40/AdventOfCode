
/// Trait defining the interface for solutions to each day's challenge
pub trait Solution {
    fn part1(&mut self) -> String;
    fn part2(&mut self) -> String;
}

/// Factory trait to initialize solutions
pub trait SolutionFactory {
    fn init(inputs_dir : &str, day : u8) -> Box<dyn Solution>;
}

/// Utility function to read input file for a given day
pub fn read_input(inputs_dir : &str, day : u8) -> Vec<u8> {
    let input_path = format!("{}/day{:02}.txt", inputs_dir, day);
    std::fs::read(&input_path).expect(&format!("Failed to read input file: {}", input_path))
}