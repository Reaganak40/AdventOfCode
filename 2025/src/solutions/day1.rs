use crate::solution_traits::{read_input, Solution, SolutionFactory};

type DialNumT = u16;
const DIAL_MAX: DialNumT = 100;
const DIAL_START: DialNumT = 50;

pub enum Rotation {
    /// Dial rotation left (this many digits)
    Left(DialNumT),
    /// Dial rotation right (this many digits)
    Right(DialNumT),
}

impl Rotation {
    pub fn parse_rotation_file(input: String) -> Vec<Rotation> {
        input
            .lines()
            .filter(|line| !line.is_empty())
            .map(|rotation_str| Rotation::from(rotation_str))
            .collect()
    }

    pub fn apply(&self, dial: DialNumT) -> DialNumT {
        match self {
            Rotation::Left(count) => {
                let real_diff = count % DIAL_MAX;
                if real_diff > dial {
                    DIAL_MAX - (real_diff - dial)
                } else {
                    dial - real_diff
                }
            }
            Rotation::Right(count) => {
                let real_diff = count % DIAL_MAX;
                (dial + real_diff) % DIAL_MAX
            }
        }
    }

    pub fn get_clicks(&self, dial: DialNumT) -> u32 {
        match self {
            Rotation::Left(count) => {
                let mut clicks = count / DIAL_MAX;
                let real_diff = count % DIAL_MAX;
                if real_diff >= dial && dial != 0 {
                    clicks += 1;
                }
                clicks as u32
            }
            Rotation::Right(count) => {
                let mut clicks = count / DIAL_MAX;
                let real_diff = count % DIAL_MAX;
                if real_diff + dial >= DIAL_MAX {
                    clicks += 1
                }
                clicks as u32
            }
        }
    }
}

impl From<&str> for Rotation {
    fn from(value: &str) -> Self {
        let direction = value
            .bytes()
            .nth(0)
            .expect("Failed to get first byte (empty string)");
        let count_substr = value
            .get(1..)
            .expect("Failed to create count substring from rotation string");
        let count: DialNumT = count_substr.parse().expect(&format!(
            "Failed to parse rotation string count: {}",
            count_substr
        ));

        match char::from(direction) {
            'L' => Rotation::Left(count),

            'R' => Rotation::Right(count),

            _ => {
                panic!("Failed to get valid direction from string: {}", value);
            }
        }
    }
}

pub struct Day1 {
    rotations: Vec<Rotation>,
}

impl Solution for Day1 {
    fn part1(&mut self) -> String {
        let mut current = DIAL_START;
        let mut occurances_of_zero: u32 = 0;
        self.rotations.iter().for_each(|rotation| {
            current = rotation.apply(current);
            if current == 0 {
                occurances_of_zero += 1;
            }
        });

        occurances_of_zero.to_string()
    }

    fn part2(&mut self) -> String {
        let mut current = DIAL_START;
        let mut occurances_of_zero: u32 = 0;
        self.rotations.iter().for_each(|rotation| {
            occurances_of_zero += rotation.get_clicks(current);
            current = rotation.apply(current);
        });

        occurances_of_zero.to_string()
    }
}

impl SolutionFactory for Day1 {
    fn init(inputs_dir: &str, day: u8) -> Box<dyn Solution> {
        let _debug_file = Some(String::from("sample.txt"));
        let input_bytes = read_input(inputs_dir, day, None);
        let input = String::from_utf8_lossy(&input_bytes).to_string();
        let rotations = Rotation::parse_rotation_file(input);

        Box::new(Day1 { rotations })
    }
}
