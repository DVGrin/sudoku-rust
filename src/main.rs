use std::io;
use std::collections::HashMap;

fn parse_input() -> HashMap<String, String> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)
               .expect("Failed to read input puzzle");
    assert_eq!(input.trim().chars().count(), 81, "Input puzzle should have exactly 81 characters in it");

    let mut split_input = [""; 9];
    for i in 0..9 {
        split_input[i] = &input[i * 9 .. (i+1) * 9];
    }

    const ROWS: &str = "ABCDEFGHI";
    const COLUMNS: &str = "123456789";
    let mut squares: HashMap<String, String> = HashMap::new();
    for (i, row) in ROWS.chars().enumerate() {
        for (input_char, column) in split_input[i].chars().zip(COLUMNS.chars()) {
            let value = match input_char {
                '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => input_char.to_string(),
                '.' => String::from("123456789"),
                _ => unreachable!()
            };
            let key = format!("{}{}", row, column);
            squares.insert(key, value);
        }
    }
    return squares;
}

fn main() {
    // let sudoku_field = parse_input();
    parse_input();
}
