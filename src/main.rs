use array_init::array_init;
use crate::Cell::{Given, Guesses};

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
    Guesses([bool;GRID_SIZE])
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
struct Puzzle {
    cells: [[Cell;GRID_SIZE];GRID_SIZE],
}

impl Puzzle {
    fn update_row(&mut self, index: usize) {
        if let Some(row) = self.cells.get_mut(index) {
            let mut cell_mask = [true;GRID_SIZE];
            for cell in row.iter() {
                if let Given(cell) = cell {
                    if let Some(guess) = cell_mask.get_mut(cell.value as usize) {
                        *guess = false;
                    }
                }
            }
            for cell in row.iter_mut() {
                if let Guesses(guesses) = cell {
                    guesses.iter_mut().zip(cell_mask.iter_mut()).for_each(|(guess, mask)| *guess &= *mask)
                }
            }
        }
    }
}

impl From<String> for Puzzle {
    fn from(input: String) -> Self {
        let chars = input.chars().collect::<Vec<_>>();
        Self {
            cells: array_init(|row| array_init::<[Cell;GRID_SIZE], _>(|column| match chars.get(9 * row + column) {
                Some('1') => Given(SolvedCell {value: 0, given: true }),
                Some('2') => Given(SolvedCell {value: 1, given: true }),
                Some('3') => Given(SolvedCell {value: 2, given: true }),
                Some('4') => Given(SolvedCell {value: 3, given: true }),
                Some('5') => Given(SolvedCell {value: 4, given: true }),
                Some('6') => Given(SolvedCell {value: 5, given: true }),
                Some('7') => Given(SolvedCell {value: 6, given: true }),
                Some('8') => Given(SolvedCell {value: 7, given: true }),
                Some('9') => Given(SolvedCell {value: 8, given: true }),
                _ => Guesses([true;GRID_SIZE]),
            }
            )),
        }
    }
}

fn main() {
    let mut puzzle: Puzzle = String::from("100009006020700050003080400009400003080050100700006020000003700000020080000100009").into();
    println!("{:?}", puzzle);
    puzzle.update_row(0);
    println!("{:?}", puzzle);
}
