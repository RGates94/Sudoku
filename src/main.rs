use array_init::array_init;
use crate::Cell::{Given, Guesses};

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
    cells: [[Cell;GRID_SIZE];GRID_SIZE]
}

impl From<String> for Puzzle {
    fn from(input: String) -> Self {
        let chars = input.chars().collect::<Vec<_>>();
        Self {
            cells: array_init(|row| array_init::<[Cell;GRID_SIZE], _>(|column| match chars.get(9 * row + column) {
                Some('1') => Given(1),
                Some('2') => Given(2),
                Some('3') => Given(3),
                Some('4') => Given(4),
                Some('5') => Given(5),
                Some('6') => Given(6),
                Some('7') => Given(7),
                Some('8') => Given(8),
                Some('9') => Given(9),
                _ => Guesses([true;9]),
            })),
        }
    }
}

fn main() {
    let puzzle: Puzzle = String::from("100009006020700050003080400009400003080050100700006020000003700000020080000100009").into();
    println!("{:?}", puzzle);
}
