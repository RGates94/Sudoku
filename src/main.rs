use array_init::array_init;
use crate::Cell::{Given, _Solved, Guesses};

const BOX_SIZE: usize = 3;
const GRID_SIZE: usize = BOX_SIZE * BOX_SIZE;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
enum Cell {
    Given(u8),
    _Solved(u8),
    Guesses([bool;GRID_SIZE])
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
struct Puzzle {
    cells: [[Cell;GRID_SIZE];GRID_SIZE],
}

impl Puzzle {
    fn update_row(&mut self, index: usize) {
        let mut cell_mask = [true;GRID_SIZE];
        if let Some(row) = self.cells.get_mut(index) {
            for cell in row.iter_mut() {
                match *cell {
                    Given(val) => {
                        if let Some(guess) = cell_mask.get_mut(val as usize) {
                            *guess = false;
                        }
                    },
                    _Solved(val) => {
                        if let Some(guess) = cell_mask.get_mut(val as usize) {
                            *guess = false;
                        }
                    }
                    _ => {}
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
                Some('1') => Given(0),
                Some('2') => Given(1),
                Some('3') => Given(2),
                Some('4') => Given(3),
                Some('5') => Given(4),
                Some('6') => Given(5),
                Some('7') => Given(6),
                Some('8') => Given(7),
                Some('9') => Given(8),
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
