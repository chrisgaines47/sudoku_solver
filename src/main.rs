//stores an individual SudokuBox
struct SudokuBox {}

//stores a 3x3 grid of SudokuBoxes
struct SudokuSquare {}

//stores a 3x3 grid of SudokuSquares
struct SudokuPuzzle {}

//reads an input file and returns a created SudokuPuzzle
fn get_sudoku_puzzle() -> SudokuPuzzle {
    return SudokuPuzzle {};
}

//takes an unsolved sudoku puzzle as input and returns a solved sudoku puzzle as output
fn solve_sudoku_puzzle(_unsolved_sudoku_puzzle: SudokuPuzzle) -> SudokuPuzzle {
    let solved_sudoku_puzzle = _unsolved_sudoku_puzzle;
    return solved_sudoku_puzzle;
}

//takes a solved SudokuPuzzle and writes it to an output file
fn output_solved_puzzle(_solved_sudoku_puzzle: SudokuPuzzle) {
    //output to file here
}

fn main() {
    println!("Sudoku Solver: start");

    let sudoku_puzzle = get_sudoku_puzzle();
    let solved_sudoku_puzzle = solve_sudoku_puzzle(sudoku_puzzle);
    output_solved_puzzle(solved_sudoku_puzzle);
}
