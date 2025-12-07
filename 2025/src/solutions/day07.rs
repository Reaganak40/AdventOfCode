use core::panic;

use crate::solution_traits::{read_input, Solution, SolutionFactory};

#[derive(Clone, Copy)]
enum GridItem {
    /// Entry point for tachyon manifold beams
    TachyonManifold,

    /// Nothing here
    EmptySpace,

    /// Beam moving downwards, tracks how many timelines this beam spawned from
    TachyonBeam { timelines: u64 },

    /// Splits a beam into two
    Splitter,
}

impl GridItem {
    pub fn is_splitter(&self) -> bool {
        matches!(self, GridItem::Splitter)
    }

    pub fn is_empty_space(&self) -> bool {
        matches!(self, GridItem::EmptySpace)
    }

    pub fn is_beam(&self) -> bool {
        matches!(self, GridItem::TachyonBeam { .. })
    }

    /// If this is a beam, how many timelines it has, otherwise returns 0
    pub fn num_timelines(&self) -> u64 {
        if let GridItem::TachyonBeam { timelines } = self {
            *timelines
        } else {
            0
        }
    }
}

impl From<char> for GridItem {
    fn from(value: char) -> Self {
        match value {
            'S' => GridItem::TachyonManifold,
            '.' => GridItem::EmptySpace,
            '^' => GridItem::Splitter,
            _ => panic!("Got undefined char for GridItem: {}", value),
        }
    }
}

type GridRow = Vec<GridItem>;
type Grid = Vec<GridRow>;

fn create_grid_row(line: &str) -> GridRow {
    line.chars().map(|c| GridItem::from(c)).collect()
}

struct TeleportRoom {
    grid: Grid,

    timelines: Option<u64>,
}

impl TeleportRoom {
    pub fn from_file(input: &str) -> TeleportRoom {
        let grid = input
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| create_grid_row(line))
            .collect();

        TeleportRoom {
            grid,
            timelines: None,
        }
    }

    fn get_item(&self, x: usize, y: usize) -> GridItem {
        self.grid[y][x]
    }

    /// Sets the item at this location to a beam, if there is no existing beam sets the number
    /// of timelines to the beam count, otherwise adds to the beam count
    /// 
    /// If the item at this location is not empty space or not a beam, does nothing
    fn add_beam(&mut self, x: usize, y: usize, beam_count: u64) {
        if let Some(row) = self.grid.get_mut(y) {
            if let Some(item) = row.get_mut(x) {
                if item.is_empty_space() {
                    *item = GridItem::TachyonBeam {
                        timelines: beam_count,
                    }
                } else if let GridItem::TachyonBeam { timelines } = item {
                    *timelines += beam_count
                }
            }
        }
    }

    fn width(&self) -> usize {
        self.grid[0].len()
    }

    fn height(&self) -> usize {
        self.grid.len()
    }

    pub fn simulate(&mut self) -> usize {
        let mut used_splinters = 0;
        let max_row_idx = self.height() - 1;
        let width = self.width();

        for y in 0..max_row_idx {
            for x in 0..width {
                let item = self.get_item(x, y);
                match item {
                    GridItem::EmptySpace => { /* Do nothing */ }
                    GridItem::TachyonManifold => {
                        // Spawn beam
                        self.add_beam(x, y + 1, 1);
                    }
                    GridItem::TachyonBeam { timelines } => {
                        // move beam downward
                        let item_below_beam = self.get_item(x, y + 1);
                        if item_below_beam.is_splitter() {
                            self.add_beam(x - 1, y + 1, timelines);
                            self.add_beam(x + 1, y + 1, timelines);
                        } else {
                            self.add_beam(x, y + 1, timelines);
                        }
                    }

                    GridItem::Splitter => {
                        if self.get_item(x, y - 1).is_beam() {
                            used_splinters += 1
                        }
                    }
                }
            }
        }

        let timelines: u64 = self
            .grid
            .last()
            .unwrap()
            .iter()
            .map(|item| item.num_timelines())
            .sum();
        self.timelines = Some(timelines);
        used_splinters
    }
}

pub struct Day7 {
    teleport_room: TeleportRoom,
}

impl Solution for Day7 {
    fn part1(&mut self) -> String {
        let beam_splits = self.teleport_room.simulate();
        beam_splits.to_string()
    }

    fn part2(&mut self) -> String {
        let timelines = self
            .teleport_room
            .timelines
            .expect("No timeline set (simulation was not run)");
        timelines.to_string()
    }
}

impl SolutionFactory for Day7 {
    fn init(inputs_dir: &str, day: u8) -> Box<dyn Solution> {
        let _debug_file = Some(String::from("sample.txt"));
        let input_bytes = read_input(inputs_dir, day, None);
        let input = String::from_utf8_lossy(&input_bytes).to_string();

        let teleport_room = TeleportRoom::from_file(&input);
        Box::new(Day7 { teleport_room })
    }
}
