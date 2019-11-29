use crate::Cell::{Candidates, Given};
use array_init::array_init;

const BOX_SIZE: usize = 3;
const GRID_SIZE: usize = BOX_SIZE * BOX_SIZE;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
struct SolvedCell {
    value: u8,
    given: bool,
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
enum Cell {
    Given(SolvedCell),
    Candidates([bool; GRID_SIZE]),
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
struct Puzzle {
    cells: [[Cell; GRID_SIZE]; GRID_SIZE],
}

impl Puzzle {
    fn update_row(&mut self, index: usize) {
        if let Some(row) = self.cells.get_mut(index) {
            let mut cell_mask = [true; GRID_SIZE];
            for cell in row.iter() {
                if let Given(cell) = cell {
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
                if let Given(cell) = cell {
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
                if let Given(cell) = cell {
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
    fn update_candidates(&mut self) {
        for i in 0..GRID_SIZE {
            self.update_row(i);
            self.update_column(i);
            self.update_box(i);
        }
    }
    fn recursive_solve(&mut self, output: &mut Vec<Self>, max_solutions: usize) {
        self.update_candidates();
        let mut min_options = GRID_SIZE;
        let mut best_cell = 0;
        let mut solved = true;
        for (index, cell) in self.cells.iter().flatten().enumerate() {
            if let Candidates(candidates) = cell {
                solved = false;
                if min_options >= candidates.iter().filter(|x| **x).count() {
                    min_options = candidates.iter().count();
                    best_cell = index;
                }
            }
        }
        if solved {
            output.push(self.clone());
            return;
        }
        let candidates = self.cells[best_cell / GRID_SIZE][best_cell % GRID_SIZE];
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
                new_grid.cells[best_cell / GRID_SIZE][best_cell % GRID_SIZE] =
                    Cell::Given(SolvedCell {
                        value: option as u8,
                        given: false,
                    });
                new_grid.recursive_solve(output, max_solutions);
            }
        }
    }
    fn solved_string(&self) -> String {
        let mut output = String::new();
        for cell in self.cells.iter().flatten() {
            output += &match cell {
                Given(solved) => (solved.value + 1).to_string(),
                Candidates(_) => "0".to_string(),
            };
        }
        output
    }
}

impl From<String> for Puzzle {
    fn from(input: String) -> Self {
        let chars = input.chars().collect::<Vec<_>>();
        Self {
            cells: array_init(|row| {
                array_init::<[Cell; GRID_SIZE], _>(|column| match chars.get(9 * row + column) {
                    Some('1') => Given(SolvedCell {
                        value: 0,
                        given: true,
                    }),
                    Some('2') => Given(SolvedCell {
                        value: 1,
                        given: true,
                    }),
                    Some('3') => Given(SolvedCell {
                        value: 2,
                        given: true,
                    }),
                    Some('4') => Given(SolvedCell {
                        value: 3,
                        given: true,
                    }),
                    Some('5') => Given(SolvedCell {
                        value: 4,
                        given: true,
                    }),
                    Some('6') => Given(SolvedCell {
                        value: 5,
                        given: true,
                    }),
                    Some('7') => Given(SolvedCell {
                        value: 6,
                        given: true,
                    }),
                    Some('8') => Given(SolvedCell {
                        value: 7,
                        given: true,
                    }),
                    Some('9') => Given(SolvedCell {
                        value: 8,
                        given: true,
                    }),
                    _ => Candidates([true; GRID_SIZE]),
                })
            }),
        }
    }
}

fn main() {
    let mut puzzle: Puzzle = String::from(
        "100009006020700050003080400009400003080050100700006020000003700000020080000100009",
    )
    .into();
    println!("{:?}", puzzle);
    let mut result = vec![];
    puzzle.recursive_solve(&mut result, 1);
    println!("{}", result[0].solved_string());
}
