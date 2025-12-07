use crate::solution_traits::{read_input, Solution, SolutionFactory};

struct BatteryBank {
    line: String,
}

type ValueRow = Vec<Option<u64>>;
type ValueTable = Vec<ValueRow>;

/// 11 instead of 12, since we exclude the first digit
const TABLE_ROW_COUNT: usize = 11;

impl BatteryBank {
    pub fn from_file(data: &str) -> Vec<BatteryBank> {
        data.lines()
            .filter(|line| !line.is_empty())
            .map(|line_str| BatteryBank::from(line_str))
            .collect()
    }

    pub fn biggest_joltage(&self) -> u32 {
        let mut digit_1_idx = self.line.len() - 2;
        let mut digit_1 = self.line.chars().nth(digit_1_idx).unwrap();

        self.line
            .chars()
            .rev()
            .skip(1)
            .enumerate()
            .for_each(|(idx, digit)| {
                if digit >= digit_1 {
                    digit_1 = digit;
                    digit_1_idx = self.line.len() - idx - 2;
                }
            });

        let mut digit_2 = self.line.chars().nth(digit_1_idx + 1).unwrap();
        self.line.chars().skip(digit_1_idx + 1).for_each(|digit| {
            digit_2 = digit_2.max(digit);
        });

        let joltage = format!("{}{}", digit_1, digit_2)
            .parse()
            .expect("Failed to parse joltage from digits");
        joltage
    }

    pub fn even_more_joltage(&self) -> u64 {
        let num_cols = self.line.len();

        // dynamic programming solution
        // value table is a 2D DP table, where the row index represents the starting digit for the sequence,
        // and the column index is how many digits we have left to use

        let mut value_table: ValueTable = (0..TABLE_ROW_COUNT)
            .into_iter()
            .map(|_| vec![None; num_cols])
            .collect();

        // start at the bottom right of the table
        for curr_idx in (0..(self.line.len())).into_iter().rev() {
            self.populate_entries(curr_idx, &mut value_table);
        }

        let mut max_joltage = 0;
        value_table[0].iter().for_each(|value| {
            if let Some(value) = value {
                max_joltage = max_joltage.max(*value);
            }
        });

        // println!("{}", max_joltage);
        // for c in self.line.chars() {
        //     print!("{:12} ", c);
        // }
        // println!("");

        // for row in value_table.iter() {
        //     for entry in row.iter() {
        //         match entry {
        //             Some(v) => print!("{:12} ", v),
        //             None => print!("{:12} ", "-"),
        //         }
        //     }
        //     println!("");
        // }

        max_joltage
    }

    fn populate_entries(
        &self,
        curr_idx: usize,
        value_table: &mut ValueTable,
    ) {
        let mut row_idx = TABLE_ROW_COUNT - 1;
        let this_row_digit = self.line.chars().nth(curr_idx).unwrap();

        loop {
            // must pick 1 digit, just find the biggest next one
            if row_idx == (TABLE_ROW_COUNT - 1) {
                let mut next_char_idx = curr_idx + 1;
                if next_char_idx >= self.line.len() {
                    row_idx -= 1;
                    continue;
                }

                let mut next_char = self.line.chars().nth(next_char_idx).unwrap();
                self.line
                    .chars()
                    .skip(next_char_idx)
                    .enumerate()
                    .for_each(|(idx, digit)| {
                        if digit > next_char {
                            next_char = digit;
                            next_char_idx = idx + curr_idx;
                        }
                    });

                let entry: u64 = format!("{}{}", this_row_digit, next_char).parse().unwrap();
                value_table[row_idx][curr_idx] = Some(entry);
            }
            // Use next row to determine best next digit
            else {
                let row_to_check = row_idx + 1;
                let mut max_value = 0;
                let mut max_value_idx = None;
                value_table[row_to_check]
                    .iter()
                    .skip(curr_idx + 1)
                    .enumerate()
                    .for_each(|(idx, value)| {
                        if let Some(value) = &value {
                            max_value = max_value.max(*value);
                            max_value_idx = Some(idx + row_idx + 1);
                        }
                    });

                if max_value_idx.is_some() {
                    let entry: u64 = format!("{}{}", this_row_digit, max_value.to_string())
                        .parse()
                        .unwrap();
                    value_table[row_idx][curr_idx] = Some(entry);
                }
            }

            if row_idx == 0 {
                break;
            }
            row_idx -= 1;
        }
    }
}

impl From<&str> for BatteryBank {
    fn from(value: &str) -> Self {
        let trimmed_line = value.trim();

        if trimmed_line.as_bytes().iter().any(|digit| {
            let c = char::from(*digit);
            c < '0' || c > '9'
        }) {
            panic!("Invalid character found in line: {}", trimmed_line);
        }

        assert!(!trimmed_line.is_empty(), "Line was empty");
        BatteryBank {
            line: String::from(trimmed_line),
        }
    }
}

pub struct Day3 {
    banks: Vec<BatteryBank>,
}

impl Solution for Day3 {
    fn part1(&mut self) -> String {
        let sum_of_joltages: u32 = self.banks.iter().map(|bank| bank.biggest_joltage()).sum();
        sum_of_joltages.to_string()
    }

    fn part2(&mut self) -> String {
        let sum_of_joltages: u64 = self.banks.iter().map(|bank| bank.even_more_joltage()).sum();
        sum_of_joltages.to_string()
    }
}

impl SolutionFactory for Day3 {
    fn init(inputs_dir: &str, day: u8) -> Box<dyn Solution> {
        let _debug_file = Some(String::from("sample.txt"));
        let input_bytes = read_input(inputs_dir, day, None);
        let input = String::from_utf8_lossy(&input_bytes).to_string();

        let banks = BatteryBank::from_file(&input);
        Box::new(Day3 { banks })
    }
}
