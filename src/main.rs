use std::io;
use std::collections::{HashMap, HashSet};

const ROWS: &str = "ABCDEFGHI";
const COLUMNS: &str = "123456789";

// 4.....8.5.3..........7......2.....6.....8.4......1.......6.3.7.5..2.....1.4......

struct Cell {
    values: String,
    units: [Vec<String>; 3],
    peers: Vec<String>
}

fn parse_input() -> Option<HashMap<String, Cell>> {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => (),
        Err(_) => {
            println!("There was a problem with reading input");
            return None
        }
    }
    if input.trim().chars().count() != 81 {
        println!("The length of puzzle should be exactly 81 characters, not {}", input.trim().chars().count());
        return None
    }

    let mut split_input = [""; 9];
    for i in 0..9 {
        split_input[i] = &input[i * 9 .. (i+1) * 9];
    }

    let mut squares: HashMap<String, Cell> = HashMap::new();
    for (i, row) in ROWS.chars().enumerate() {
        for (input_char, column) in split_input[i].chars().zip(COLUMNS.chars()) {
            let value = match input_char {
                '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => input_char.to_string(),
                '0' | '.' => String::from("123456789"),
                _ => String::from("Invalid value!")
            };
            if &value[..] == "Invalid value!" {
                println!("Invalid value in input puzzle: '{}' in position {}{}", input_char, row, column);
                return None;
            }
            let key = format!("{}{}", row, column);
            let units = find_units(&key[..]);
            let peers = find_peers(&key[..], &units);
            let value: Cell = Cell {
                values: value,
                units: units,
                peers 
            };
            squares.insert(key, value);
        }
    }
    return Some(squares);
}

fn find_units(cell: &str) -> [Vec<String>; 3] {
    let mut cell = cell.chars();
    let cell_row = cell.next().unwrap();
    let cell_column = cell.next().unwrap();

    let mut row_unit: Vec<String> = Vec::new();
    for row in ROWS.chars() {
        row_unit.push(String::from(format!("{}{}", row, cell_column)));
    }
    
    let mut column_unit: Vec<String> = Vec::new();
    for column in COLUMNS.chars() {
        column_unit.push(String::from(format!("{}{}", cell_row, column)));
    }

    let mut square_unit: Vec<String> = Vec::new();
    let rows = match cell_row {
        'A' | 'B' | 'C' => "ABC",
        'D' | 'E' | 'F' => "DEF",
        'G' | 'H' | 'I' => "GHI",
        _ => unreachable!()
    };
    let columns = match cell_column {
        '1' | '2' | '3' => "123",
        '4' | '5' | '6' => "456",
        '7' | '8' | '9' => "789",
        _ => unreachable!()
    };
    for row in rows.chars() {
        for column in columns.chars() {
            square_unit.push(String::from(format!("{}{}", row, column)))
        }
    }
    return [row_unit, column_unit, square_unit]
}

fn find_peers(cell: &str, units: &[Vec<String>; 3]) -> Vec<String> {
    let mut peers: HashSet<String> = HashSet::new();
    for unit in units.iter() {
        for value in unit.iter() {
            peers.insert(value.clone());
        }
    }
    peers.remove(&cell.to_string());
    let mut peers: Vec<String> = peers.into_iter().collect();
    peers.sort();
    return peers;
}

fn print_field(field: &HashMap<String, Cell>) {
    let mut column_width: [usize; 9] = [0; 9];
    for (j, column) in COLUMNS.chars().enumerate() {
        for row in ROWS.chars() {
            let key = format!("{}{}", row, column);
            let length = match field.get(&key) {
                Some(s) => s.values.len(),
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
                Some(s) => &s.values,
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
    let mut sudoku_field: Option<HashMap<String, Cell>> = None;
    while let None = sudoku_field {
        sudoku_field = parse_input();
    }
    let sudoku_field = sudoku_field.unwrap();
    print_field(&sudoku_field);
    let c3 = sudoku_field.get("C3").unwrap();
    println!("{:?}\n{:?}", c3.units, c3.peers);
}
