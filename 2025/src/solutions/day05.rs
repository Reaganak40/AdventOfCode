use crate::solution_traits::{read_input, Solution, SolutionFactory};

type IngredientID = u64;

#[derive(Clone, Copy)]
struct IDRange {
    start: IngredientID,
    end: IngredientID,
}

impl IDRange {
    fn contains(&self, id: IngredientID) -> bool {
        id >= self.start && id <= self.end
    }

    fn width(&self) -> u64 {
        self.end - self.start + 1
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

        let start: u64 = start.parse().expect(&format!(
            "Failed to read start value: {} from {}",
            start, value
        ));
        let end: u64 = end.parse().expect(&format!(
            "Failed to end start value: {} from {}",
            end, value
        ));

        assert!(start <= end, "Invalid range: start not less than or equal to end: {}-{}", start, end);
        IDRange { start, end }
    }
}

#[derive(Default)]
struct InventoryManagementSystem {
    ranges: Vec<IDRange>,
    available_ids: Vec<IngredientID>,
}

impl InventoryManagementSystem {
    fn add_range(&mut self, n_range: IDRange) {
        let insert_idx = self.ranges.iter().position(|existing_range| {
            if n_range.start < existing_range.start {
                true
            } else {
                false
            }
        });

        match insert_idx {
            Some(idx) => self.ranges.insert(idx, n_range),
            None => self.ranges.push(n_range),
        }
    }

    fn add_ingredient(&mut self, ingredient: IngredientID) {
        self.available_ids.push(ingredient);
    }

    fn from_file(data: &str) -> InventoryManagementSystem {
        let mut ims = InventoryManagementSystem::default();
        let mut parsing_ranges = true;
        data.lines().map(|line| line.trim()).for_each(|line| {
            if line.is_empty() {
                assert!(parsing_ranges, "Got multiple empty lines in file");
                parsing_ranges = false;
                return;
            }

            if parsing_ranges {
                let range = IDRange::from(line);
                ims.add_range(range);
            } else {
                let ingredient: IngredientID = line.parse().unwrap();
                ims.add_ingredient(ingredient);
            }
        });

      
        ims.prune();
        ims
    }

    fn fresh_ingredients(&self) -> usize {
        self.available_ids
            .iter()
            .filter(|&&id| {
                for range in self.ranges.iter() {
                    if range.contains(id) {
                        return true;
                    }
                }

                false
            })
            .count()
    }

    /// Removes all overlapping ranges into a consolidated larger range
    fn prune(&mut self) {
        let mut current_i = 0;
        while current_i < self.ranges.len() {
            let mut absorbant_range = self.ranges[current_i];
            let mut indexes_to_remove = vec![];

            for (j, other_range) in self.ranges.iter().skip(current_i + 1).enumerate() {
                if other_range.start <= absorbant_range.end {
                    absorbant_range.end = absorbant_range.end.max(other_range.end);
                    let actual_index = j + current_i + 1;
                    indexes_to_remove.push(actual_index);
                } else {
                    break;
                }
            }

            indexes_to_remove.iter().rev().for_each(|index| {
                self.ranges.remove(*index);
            });
            self.ranges[current_i] = absorbant_range;
            current_i += 1;
        }

        // sanity check to verify no overlapping
        for (i, range) in self.ranges.iter().take(self.ranges.len() - 1).enumerate() {
            for other_range in self.ranges.iter().skip(i + 1) {
                assert!(
                    range.end < other_range.start,
                    "Got overlapping ranges after prune"
                );
            }
        }
    }

    fn actual_fresh_ingredients_according_to_ranges(&self) -> u64 {
        self.ranges.iter().map(|range| range.width()).sum()
    }
}

pub struct Day5 {
    ims: InventoryManagementSystem,
}

impl Solution for Day5 {
    fn part1(&mut self) -> String {
        let fresh_ingredients = self.ims.fresh_ingredients();
        fresh_ingredients.to_string()
    }

    fn part2(&mut self) -> String {
        let fresh_ingredients = self.ims.actual_fresh_ingredients_according_to_ranges();
        fresh_ingredients.to_string()
    }
}

impl SolutionFactory for Day5 {
    fn init(inputs_dir: &str, day: u8) -> Box<dyn Solution> {
        let _debug_file = Some(String::from("sample.txt"));
        let input_bytes = read_input(inputs_dir, day, None);
        let input = String::from_utf8_lossy(&input_bytes).to_string();

        let ims = InventoryManagementSystem::from_file(&input);

        Box::new(Day5 { ims })
    }
}
