use std::mem;

use crate::solution_traits::{read_input, Solution, SolutionFactory};

#[derive(Clone, Copy)]
enum Operation {
    Add,
    Multiply,
}

impl From<&str> for Operation {
    fn from(value: &str) -> Self {
        assert!(
            value.len() == 1,
            "Operator string must be a single char, got: {}",
            value
        );
        match value.chars().nth(0).unwrap() {
            '+' => Operation::Add,
            '*' => Operation::Multiply,
            _ => panic!("Got undefined operator: {}", value),
        }
    }
}

struct Equation {
    nums: Vec<u64>,
    operation: Operation,
}

impl Equation {
    pub fn cephalopods_format_1(data: &str) -> Vec<Equation> {
        let lines: Vec<&str> = data.lines().filter(|line| !line.is_empty()).collect();

        // get all the number rows
        let rows: Vec<Vec<u64>> = lines
            .iter()
            .take(lines.len() - 1)
            .map(|line| {
                let nums: Vec<u64> = line
                    .split(" ")
                    .filter_map(|number| {
                        let number = number.trim();
                        if number.is_empty() {
                            return None;
                        }

                        let number: u64 = number
                            .parse()
                            .expect(&format!("Invalid number: {}", number));
                        Some(number)
                    })
                    .collect();
                nums
            })
            .collect();

        // parse the operator row
        let operators: Vec<Operation> = lines
            .last()
            .unwrap()
            .split(" ")
            .filter_map(|operator| {
                let operator = operator.trim();
                if operator.is_empty() {
                    return None;
                }

                Some(Operation::from(operator))
            })
            .collect();

        // sanity check that all rows are of equal size
        let fixed_size = rows.get(0).unwrap().len();
        rows.iter().skip(1).for_each(|row| {
            assert_eq!(fixed_size, row.len());
        });
        assert_eq!(operators.len(), fixed_size);

        let mut equations = Vec::with_capacity(fixed_size);
        for col in 0..fixed_size {
            let nums: Vec<u64> = rows.iter().map(|row| row[col]).collect();

            let operation = operators[col];

            equations.push(Equation { nums, operation })
        }

        equations
    }

    pub fn cephalopods_format_2(data: &str) -> Vec<Equation> {
        let lines: Vec<&str> = data.lines().filter(|line| !line.is_empty()).collect();

        // parse the operator row
        let operators: Vec<Operation> = lines
            .last()
            .unwrap()
            .split(" ")
            .filter_map(|operator| {
                let operator = operator.trim();
                if operator.is_empty() {
                    return None;
                }

                Some(Operation::from(operator))
            })
            .collect();

        // sanity check that all rows are of equal size
        let fixed_size = lines.get(0).unwrap().len();
        lines.iter().skip(1).for_each(|row| {
            assert_eq!(fixed_size, row.len());
        });

        let mut equations = Vec::with_capacity(operators.len());
        let mut nums = vec![];
        let mut operator_idx = 0;
        for col in 0..fixed_size {
            let digits: String = lines
                .iter()
                .take(lines.len() - 1)
                .filter_map(|line| {
                    if let Some(digit) = line.chars().nth(col) {
                        if !digit.is_whitespace() {
                            return Some(digit);
                        }
                    }

                    None
                })
                .collect();

            if digits.is_empty() {
                let operation = operators[operator_idx];
                operator_idx += 1;
                let mut n_nums = vec![];
                mem::swap(&mut nums, &mut n_nums);
                equations.push(Equation {
                    nums: n_nums,
                    operation,
                })
            } else {
                nums.push(digits.parse().unwrap());
            }
        }

        if nums.len() > 0 {
            let operation = operators[operator_idx];
            equations.push(Equation { nums, operation })
        }

        equations
    }

    fn solve(&self) -> u64 {
        match self.operation {
            Operation::Add => self.nums.iter().sum(),
            Operation::Multiply => self.nums.iter().product(),
        }
    }
}
pub struct Day6 {
    equations_part1: Vec<Equation>,
    equations_part2: Vec<Equation>,
}

impl Solution for Day6 {
    fn part1(&mut self) -> String {
        let final_sum: u64 = self
            .equations_part1
            .iter()
            .map(|equation| equation.solve())
            .sum();
        final_sum.to_string()
    }

    fn part2(&mut self) -> String {
        let final_sum: u64 = self
            .equations_part2
            .iter()
            .map(|equation| equation.solve())
            .sum();
        final_sum.to_string()
    }
}

impl SolutionFactory for Day6 {
    fn init(inputs_dir: &str, day: u8) -> Box<dyn Solution> {
        let _debug_file = Some(String::from("sample.txt"));
        let input_bytes = read_input(inputs_dir, day, None);
        let input = String::from_utf8_lossy(&input_bytes).to_string();

        let equations_part1 = Equation::cephalopods_format_1(&input);
        let equations_part2 = Equation::cephalopods_format_2(&input);

        Box::new(Day6 {
            equations_part1,
            equations_part2,
        })
    }
}
