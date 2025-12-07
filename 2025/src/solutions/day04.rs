use crate::solution_traits::{read_input, Solution, SolutionFactory};

const ACCESSIBLE_LIMIT: usize = 4;

#[repr(u8)]
#[derive(Clone, Copy)]
enum GridSlot {
    /// Slot has nothing in it
    Empty = 0,

    /// Slot is taken by a roll of paper
    RollOfPaper = 1,
}

impl GridSlot {
    pub fn is_roll_of_paper(&self) -> bool {
        matches!(self, GridSlot::RollOfPaper)
    }

    pub fn remove_paper(&mut self) {
        *self= GridSlot::Empty;
    }
}

impl From<char> for GridSlot {
    fn from(value: char) -> Self {
        match value {
            '.' => GridSlot::Empty,
            '@' => GridSlot::RollOfPaper,
            _ => panic!("Invalid value for grid slot: {}", value),
        }
    }
}

type GridRow = Vec<GridSlot>;
fn create_grid_row(line: &str) -> GridRow {
    line.chars().map(|c| GridSlot::from(c)).collect()
}

struct Grid {
    rows: Vec<GridRow>,
}

impl Grid {
    pub fn from_file(data: &str) -> Grid {
        let rows = data
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| create_grid_row(line))
            .collect();

        Grid { rows }
    }

    pub fn width(&self) -> usize {
        self.rows[0].len()
    }

    pub fn height(&self) -> usize {
        self.rows.len()
    }

    pub fn accessible_rolls_of_paper(&self) -> usize {
        let mut count = 0;
        let max_x = self.width();
        let max_y = self.height();

        for y in 0..max_y {
            for x in 0..max_x {
                if self.is_accessible(x as i32, y as i32) {
                    count += 1;
                }
            }
        }

        count
    }

    fn get(&self, x: i32, y: i32) -> Option<GridSlot> {
        if x < 0 || y < 0 {
            return None;
        }

        let x = x as usize;
        let y = y as usize;

        if x >= self.width() || y >= self.height() {
            return None;
        }

        Some(self.rows[y][x])
    }

    fn is_accessible(&self, x: i32, y: i32) -> bool {
        let slot = match self.get(x, y) {
            Some(s) => s,
            None => return false,
        };

        match slot {
            GridSlot::Empty => false,
            GridSlot::RollOfPaper => {
                let mut adjacent = 0;

                let coords = vec![
                    (x - 1, y - 1),
                    (x, y - 1),
                    (x + 1, y - 1),
                    (x - 1, y),
                    (x + 1, y),
                    (x - 1, y + 1),
                    (x, y + 1),
                    (x + 1, y + 1),
                ];
                assert_eq!(coords.len(), 8);

                coords.iter().for_each(|(x, y)| {
                    if let Some(slot) = self.get(*x, *y) {
                        if slot.is_roll_of_paper() {
                            adjacent += 1;
                        }
                    }
                });

                adjacent < ACCESSIBLE_LIMIT
            }
        }
    }

    fn cleanup(&mut self) -> usize {
        let mut count = 0;
        let max_x = self.width();
        let max_y = self.height();


        loop {
            let original_count = count;
            for y in 0..max_y {
                for x in 0..max_x {
                    if self.is_accessible(x as i32, y as i32) {
                        count += 1;
                        self.remove_paper(x, y);
                    }
                }
            }

            if original_count == count {
                break
            }
        }

        count
    }

    fn remove_paper(&mut self, x : usize, y : usize) {
        if let Some(row) = self.rows.get_mut(y) {
            if let Some(slot) = row.get_mut(x) {
                slot.remove_paper();
            }
        }
    }
}

pub struct Day4 {
    grid: Grid,
}

impl Solution for Day4 {
    fn part1(&mut self) -> String {
        let accessible_rolls = self.grid.accessible_rolls_of_paper();
        accessible_rolls.to_string()
    }

    fn part2(&mut self) -> String {
        let accessible_rolls = self.grid.cleanup();
        accessible_rolls.to_string()
    }
}

impl SolutionFactory for Day4 {
    fn init(inputs_dir: &str, day: u8) -> Box<dyn Solution> {
        let _debug_file = Some(String::from("sample.txt"));
        let input_bytes = read_input(inputs_dir, day, None);
        let input = String::from_utf8_lossy(&input_bytes).to_string();

        let grid = Grid::from_file(&input);

        Box::new(Day4 { grid })
    }
}
