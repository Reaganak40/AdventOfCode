
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
pub fn read_input(inputs_dir : &str, day : u8, special_file : Option<String>) -> Vec<u8> {
    let file = special_file.unwrap_or(format!("day{:02}.txt", day));
    let input_path = format!("{}/{:02}/{}", inputs_dir, day, file);
    std::fs::read(&input_path).expect(&format!("Failed to read input file: {}", input_path))
}