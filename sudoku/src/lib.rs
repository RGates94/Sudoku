use crate::Cell::{Candidates, Solved};
use array_init::array_init;
use rand::prelude::SliceRandom;
use std::ops::{Index, IndexMut};

const BOX_SIZE: usize = 3;
const GRID_SIZE: usize = BOX_SIZE * BOX_SIZE;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct SolvedCell {
    value: u8,
    given: bool,
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum Cell {
    Solved(SolvedCell),
    Candidates([bool; GRID_SIZE]),
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Puzzle {
    cells: [[Cell; GRID_SIZE]; GRID_SIZE],
}

impl Puzzle {
    fn update_row(&mut self, index: usize) {
        if let Some(row) = self.cells.get_mut(index) {
            let mut cell_mask = [true; GRID_SIZE];
            for cell in row.iter() {
                if let Solved(cell) = cell {
                    if let Some(guess) = cell_mask.get_mut(cell.value as usize) {
                        *guess = false;
                    }
                }
            }
            for cell in row.iter_mut() {
                if let Candidates(guesses) = cell {
                    guesses
                        .iter_mut()
                        .zip(cell_mask.iter_mut())
                        .for_each(|(guess, mask)| *guess &= *mask)
                }
            }
        }
    }
    fn update_column(&mut self, index: usize) {
        if index < GRID_SIZE {
            let mut cell_mask = [true; GRID_SIZE];
            for row in self.cells.iter_mut() {
                let cell = row
                    .get_mut(index)
                    .expect("Column index out of bounds unexpectedly");
                if let Solved(cell) = cell {
                    if let Some(guess) = cell_mask.get_mut(cell.value as usize) {
                        *guess = false;
                    }
                }
            }
            for row in self.cells.iter_mut() {
                if let Some(Candidates(guesses)) = row.get_mut(index) {
                    guesses
                        .iter_mut()
                        .zip(cell_mask.iter_mut())
                        .for_each(|(guess, mask)| *guess &= *mask)
                }
            }
        }
    }
    fn update_box(&mut self, index: usize) {
        if index < GRID_SIZE {
            let mut cell_mask = [true; GRID_SIZE];
            for cell in self.cells
                [BOX_SIZE * (index / BOX_SIZE)..BOX_SIZE * ((index / BOX_SIZE) + 1)]
                .iter()
                .map(|x| &x[BOX_SIZE * (index % BOX_SIZE)..BOX_SIZE * ((index % BOX_SIZE) + 1)])
                .flatten()
            {
                if let Solved(cell) = cell {
                    if let Some(guess) = cell_mask.get_mut(cell.value as usize) {
                        *guess = false;
                    }
                }
            }

            for cell in self.cells
                [BOX_SIZE * (index / BOX_SIZE)..BOX_SIZE * ((index / BOX_SIZE) + 1)]
                .iter_mut()
                .map(|x| &mut x[BOX_SIZE * (index % BOX_SIZE)..BOX_SIZE * ((index % BOX_SIZE) + 1)])
                .flatten()
            {
                if let Candidates(guesses) = cell {
                    guesses
                        .iter_mut()
                        .zip(cell_mask.iter_mut())
                        .for_each(|(guess, mask)| *guess &= *mask)
                }
            }
        }
    }
    pub fn sweep(&mut self) {
        for i in 0..GRID_SIZE {
            self.update_row(i);
            self.update_column(i);
            self.update_box(i);
        }
    }
    fn insert_value(&mut self, index: usize, value: u8) {
        if index < GRID_SIZE * GRID_SIZE {
            self[index] = Solved(SolvedCell {
                value,
                given: false,
            });
            self.update_row(index / GRID_SIZE);
            self.update_column(index % GRID_SIZE);
            self.update_box(3 * (index / (GRID_SIZE * BOX_SIZE)) + index % BOX_SIZE);
        }
    }
    pub fn recursive_solve(&mut self, output: &mut Vec<Self>, max_solutions: usize) {
        let mut min_options = GRID_SIZE;
        let mut best_cell = 0;
        let mut solved = true;
        for (index, cell) in self.cells.iter().flatten().enumerate() {
            if let Candidates(candidates) = cell {
                solved = false;
                let options = candidates.iter().filter(|x| **x).count();
                if min_options >= options {
                    min_options = options;
                    best_cell = index;
                }
            }
        }
        if solved {
            output.push(self.clone());
            return;
        }
        let candidates = self[best_cell];
        if let Candidates(candidates) = candidates {
            for option in
                candidates
                    .iter()
                    .enumerate()
                    .filter_map(|(idx, &valid)| if valid { Some(idx) } else { None })
            {
                if output.len() >= max_solutions {
                    return;
                }
                let mut new_grid = self.clone();
                new_grid.insert_value(best_cell, option as u8);
                new_grid.recursive_solve(output, max_solutions);
            }
        }
    }
    pub fn solved_string(&self) -> String {
        self.cells
            .iter()
            .flatten()
            .map(|cell| match cell {
                Solved(solved) => (solved.value + 49) as char,
                Candidates(_) => '0',
            })
            .collect()
    }
    pub fn minimize_puzzle(&mut self) {
        let mut rng = rand::thread_rng();
        let mut cells = (0..81).collect::<Vec<_>>();
        cells.shuffle(&mut rng);
        for cell_idx in cells {
            if let Solved(val) = self[cell_idx] {
                self[cell_idx] = Candidates([true; 9]);
                let mut new_grid = self.clone();
                let mut result = vec![];
                new_grid.sweep();
                new_grid.recursive_solve(&mut result, 200);
                if result.len() != 1 {
                    self[cell_idx] = Solved(val);
                }
            }
        }
    }
}

impl Index<usize> for Puzzle {
    type Output = Cell;
    fn index(&self, index: usize) -> &Self::Output {
        &self.cells[index / GRID_SIZE][index % GRID_SIZE]
    }
}

impl IndexMut<usize> for Puzzle {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.cells[index / GRID_SIZE][index % GRID_SIZE]
    }
}

impl From<&str> for Puzzle {
    fn from(input: &str) -> Self {
        let chars = input.chars().collect::<Vec<_>>();
        Self {
            cells: array_init(|row| {
                array_init::<[Cell; GRID_SIZE], _>(|column| match chars.get(9 * row + column) {
                    Some('1') => Solved(SolvedCell {
                        value: 0,
                        given: true,
                    }),
                    Some('2') => Solved(SolvedCell {
                        value: 1,
                        given: true,
                    }),
                    Some('3') => Solved(SolvedCell {
                        value: 2,
                        given: true,
                    }),
                    Some('4') => Solved(SolvedCell {
                        value: 3,
                        given: true,
                    }),
                    Some('5') => Solved(SolvedCell {
                        value: 4,
                        given: true,
                    }),
                    Some('6') => Solved(SolvedCell {
                        value: 5,
                        given: true,
                    }),
                    Some('7') => Solved(SolvedCell {
                        value: 6,
                        given: true,
                    }),
                    Some('8') => Solved(SolvedCell {
                        value: 7,
                        given: true,
                    }),
                    Some('9') => Solved(SolvedCell {
                        value: 8,
                        given: true,
                    }),
                    _ => Candidates([true; GRID_SIZE]),
                })
            }),
        }
    }
}
