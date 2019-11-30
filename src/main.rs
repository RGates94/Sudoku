use sudoku::Puzzle;
use std::time::Instant;

fn main() {
    let mut puzzle: Puzzle =
        "100009006020700050003080400009400003080050100700006020000003700000020080000100009".into();
    let start = Instant::now();
    let mut result = vec![];
    puzzle.sweep();
    puzzle.recursive_solve(&mut result, 1_000);
    let elapsed = Instant::now() - start;
    result.sort();
    let max_solutions = result.len() == 1_000;
    for solution in result {
        println!("{}", solution.solved_string());
    }
    if max_solutions {
        println!("Maxed out...");
    }
    println!(
        "Total of {}.{:0>3} seconds elapsed",
        elapsed.as_secs(),
        elapsed.subsec_millis()
    )
}
