use std::io;
use std::collections::HashMap;

const ROWS: &str = "ABCDEFGHI";
const COLUMNS: &str = "123456789";

fn parse_input() -> HashMap<String, String> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)
               .expect("Failed to read input puzzle");
    assert_eq!(input.trim().chars().count(), 81, "Input puzzle should have exactly 81 characters in it");

    let mut split_input = [""; 9];
    for i in 0..9 {
        split_input[i] = &input[i * 9 .. (i+1) * 9];
    }

    let mut squares: HashMap<String, String> = HashMap::new();
    for (i, row) in ROWS.chars().enumerate() {
        for (input_char, column) in split_input[i].chars().zip(COLUMNS.chars()) {
            let value = match input_char {
                '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => input_char.to_string(),
                '0' | '.' => String::from("123456789"),
                _ => unreachable!()
            };
            let key = format!("{}{}", row, column);
            squares.insert(key, value);
        }
    } 
    return squares;
}

fn print_field(field: HashMap<String, String>) {
    let mut column_width: [usize; 9] = [0; 9];
    for (j, column) in COLUMNS.chars().enumerate() {
        for row in ROWS.chars() {
            let key = format!("{}{}", row, column);
            let length = match field.get(&key) {
                Some(s) => s.len(),
                None => unreachable!()
            };
            if length > column_width[j] {
                column_width[j] = length;
            }
        }
    }

    for row in ROWS.chars() {
        let mut to_print = String::from(" ");
        let mut delimiter_string = String::from("-");
        for (column, width) in COLUMNS.chars().zip(column_width.iter()) {
            let key = format!("{}{}", row, column);
            let value = match field.get(&key) {
                Some(s) => s,
                None => unreachable!()
            };
            to_print.push_str(&format!("{:^width$}", value, width=width)[..]);
            delimiter_string.push_str(&format!("{:->1$}", "-", width));
            let (delimiter, cell_delimeter) = match column {
               '3' | '6' => (" | ", "-+-"),
               '9' => ("", ""),
               '1' | '2' | '4' | '5' | '7' | '8' => ("  ", "--"),
               _ => unreachable!()
            };
            to_print.push_str(delimiter);
            delimiter_string.push_str(cell_delimeter);
        }
        println!("{}", to_print);
        match row {
            'C' | 'F' => println!("{}", delimiter_string),
            _ => ()
        }
    }
}

fn main() {
    let sudoku_field = parse_input();
    print_field(sudoku_field);
}
