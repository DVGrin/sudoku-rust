use sudoku;

// 003020600900305001001806400008102900700000008006708200002609500800203009005010300

fn main() {
    let input = "4.....8.5.3..........7......2.....6.....8.4......1.......6.3.7.5..2.....1.4......";
    println!("Input: {}", input);
    sudoku::solve_from_str(input);
    println!("\nGive me another input: ");
    sudoku::solve_from_stdin();
}
