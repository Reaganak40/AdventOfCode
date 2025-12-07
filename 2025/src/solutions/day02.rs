use crate::solution_traits::{read_input, Solution, SolutionFactory};

fn is_invalid_id(id : u64) -> bool {
    let id_str = id.to_string();
    if id_str.len() % 2 != 0 {
        return false;
    }

    let mid = id_str.len() / 2;
    let left = &id_str[0..mid];
    let right = &id_str[mid..];
    left == right
}

fn is_invalid_id_2(id : u64) -> bool {
    let id_str = id.to_string();
    let mid = id_str.len() / 2;
    for slice_size in 1..=mid {
        if id_str.len() % slice_size != 0 {
            continue;
        }

        let left = &id_str[0..slice_size];
        let mut idx = slice_size;
        
        let found = loop {
            let right = &id_str[idx..(idx + slice_size)];
            if left != right {
                break false;
            }

            idx += slice_size;
            if idx >= id_str.len() {
                break true;
            }
        };

        if found {
            return true;
        }
    }

    false
}

struct IDRange {
    start: u64,
    end: u64,
}

impl IDRange {
    pub fn from_csv(data: &str) -> Vec<IDRange> {
        data.split(',')
            .filter(|line| !line.is_empty())
            .map(|id_str| IDRange::from(id_str))
            .collect()
    }

    pub fn invalid_id_sum(&self) -> usize {
        (self.start..=self.end).into_iter().filter_map(|id| {
            if is_invalid_id(id) {
                Some(id as usize)
            } else {
                None
            }
        }).sum()
    }

    pub fn invalid_id_sum_repeated(&self) -> usize {
        (self.start..=self.end).into_iter().filter_map(|id| {
            if is_invalid_id_2(id) {
                Some(id as usize)
            } else {
                None
            }
        }).sum()
    }
}

impl From<&str> for IDRange {
    fn from(value: &str) -> Self {
        let mut split_items = value.split('-');
        let start = split_items
            .next()
            .expect(&format!("Failed to get first item in ID range: {}", value))
            .trim();

        let end = split_items
            .next()
            .expect(&format!("Failed to get second item in ID range: {}", value))
            .trim();

        assert!(
            split_items.next().is_none(),
            "More than two ranges found in {}",
            value
        );

        let start: u64 = start
            .parse()
            .expect(&format!("Failed to read start value: {} from {}", start, value));
        let end: u64 = end
            .parse()
            .expect(&format!("Failed to end start value: {} from {}", end, value));
        IDRange { start, end }
    }
}

pub struct Day2 {

    id_ranges : Vec<IDRange>
}

impl Solution for Day2 {
    fn part1(&mut self) -> String {
        let invalid_count : usize = self.id_ranges.iter().map(|range| {
            range.invalid_id_sum()
        }).sum();

        invalid_count.to_string()
    }

    fn part2(&mut self) -> String {
        let invalid_count : usize = self.id_ranges.iter().map(|range| {
            range.invalid_id_sum_repeated()
        }).sum();

        invalid_count.to_string()
    }
}

impl SolutionFactory for Day2 {
    fn init(inputs_dir: &str, day: u8) -> Box<dyn Solution> {
        let _debug_file = Some(String::from("sample.txt"));
        let input_bytes = read_input(inputs_dir, day, None);
        let input = String::from_utf8_lossy(&input_bytes).to_string();
        let id_ranges = IDRange::from_csv(&input);
        Box::new(Day2 { id_ranges })
    }
}
