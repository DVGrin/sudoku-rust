use std::io;
use std::process;
use std::collections::{HashMap, HashSet};

const ROWS: &str = "ABCDEFGHI";
const COLUMNS: &str = "123456789";

#[derive(Clone)]
struct Cell {
    values: String,
    units: [Vec<String>; 3],
    peers: Vec<String>
}

impl Cell {
    fn new(key: &str) -> Cell {
        let units = find_units(key);
        let peers = find_peers(key, &units);
        Cell {
            values: "123456789".to_string(),
            units,
            peers
        }
    }
}


pub fn solve_from_str(input: &str) -> () {
    let sudoku_field = match parse_from_str(input) {
        Ok(field) => field,
        Err(e) => {
            eprintln!("Invalid input: {}", e);
            return ();
        }
    };
    match search(&sudoku_field) {
        Some(field) => {
            println!("\nThe solution:\n");
            print_field(&field);
        }
        None => {
            println!("No solution found!");
        }
    }
}


pub fn solve_from_stdin() {
    let sudoku_field = parse_from_stdin();
    match search(&sudoku_field) {
        Some(field) => {
            println!("\nThe solution:\n");
            print_field(&field);
        }
        None => {
            println!("No solution found!");
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_cell() {
        let cell = Cell::new("C3");
        assert_eq!(cell.values, String::from("123456789"));

        let expected_units: [Vec<String>; 3]  = [
            vec![String::from("A3"), String::from("B3"), String::from("C3"),
                 String::from("D3"), String::from("E3"), String::from("F3"),
                 String::from("G3"), String::from("H3"), String::from("I3")],
            vec![String::from("C1"), String::from("C2"), String::from("C3"),
                 String::from("C4"), String::from("C5"), String::from("C6"),
                 String::from("C7"), String::from("C8"), String::from("C9")],
            vec![String::from("A1"), String::from("A2"), String::from("A3"),
                 String::from("B1"), String::from("B2"), String::from("B3"),
                 String::from("C1"), String::from("C2"), String::from("C3")],
        ];
        assert_eq!(cell.units, expected_units);

        let mut expected_peers: Vec<String> = Vec::new();
        for s in ["A1", "A2", "A3", "B1", "B2", "B3", "C1", "C2", "C4", "C5",
                         "C6", "C7", "C8", "C9", "D3", "E3", "F3", "G3", "H3", "I3"].iter() {
                expected_peers.push(s.to_string());
            }
        assert_eq!(cell.peers, expected_peers);
    }
}


fn parse_from_stdin() -> HashMap<String, Cell> {
    let mut input = String::new();
    if let Err(e) = io::stdin().read_line(&mut input) {
        eprintln!("There was a problem while reading input: {}", e);
        process::exit(1);
    }
    let sudoku_field = match parse_from_str(&input[..]) {
        Ok(field) => field,
        Err(e) => {
            eprintln!("{}", e);
            parse_from_stdin()
        }
    };
    return sudoku_field;
}


fn parse_from_str(input: &str) -> Result<HashMap<String, Cell>, String> {
    let input = input.to_string();
    if input.trim().chars().count() != 81 {
        let err = format!("The length of puzzle should be exactly 81 characters, not {}", input.trim().chars().count());
        return Err(err);
    }

    let mut split_input = [""; 9];
    for i in 0..9 {
        split_input[i] = &input[i * 9 .. (i+1) * 9];
    }

    let mut squares: HashMap<String, Cell> = HashMap::new();
    for row in ROWS.chars() {
        for column in COLUMNS.chars() {
            let key = format!("{}{}", row, column);
            let cell = Cell::new(&key[..]);
            squares.insert(key, cell);
        }
    }

    for (i, row) in ROWS.chars().enumerate() {
        for (input_char, column) in split_input[i].chars().zip(COLUMNS.chars()) {
            let key = format!("{}{}", row, column);
            match input_char {
                '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                    match assign_value(&mut squares, &key[..], &input_char.to_string()) {
                        Some(_) => (),
                        None => {
                            return Err("Input puzzle contains a contradiction".to_string());
                        }
                    }
                }
                '0' | '.' => (),
                _ => {
                    let err = format!("Invalid value in input puzzle: '{}' in position {}{}", input_char, row, column);
                    return Err(err);
                }
            };
        }
    }
    return Ok(squares);
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
    peers.sort_unstable();
    return peers;
}

// assign_value returns None if there was a contradiction, assigned value otherwise
fn assign_value(field: &mut HashMap<String, Cell>, key: &str, value: &str) -> Option<String> {
    if value.len() > 1 {
        let cell = field.get_mut(&key.to_string()).unwrap();
        cell.values = value.to_string();
        return Some(value.to_string());
    }
    if value == "" {
        return None;
    }

    let cell = field.get_mut(&key.to_string()).unwrap();
    cell.values = value.to_string();
    let peers = &cell.peers.clone();
    for peer_key in peers.iter() {
        let peer = field.get_mut(peer_key).unwrap();
        let mut new_values = peer.values.clone();
        match new_values.find(value) {
            None => (),
            Some(i) => {
                new_values.remove(i);
                match assign_value(field, peer_key, &new_values[..]) {
                    None => return None,
                    Some(_) => ()
                }
            }
        }
    }
    return Some(value.to_string());
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

fn candidates (field: &HashMap<String, Cell>) -> Vec<(&String, &Cell)> {
    let mut result: Vec<(&String, &Cell)> = Vec::new();
    for (key, value) in field.iter() {
        if value.values.len() > 1 {
            result.push((key, value));
        }
    }
    result.sort_by(|x, y| x.1.values.len().cmp(&y.1.values.len()));
    return result;
}

fn search(field: &HashMap<String, Cell>) -> Option<HashMap<String, Cell>> {
    let initial_field = field.clone();
    let candidates = candidates(&field);
    if candidates.is_empty() {
        return Some(initial_field);
    }

    let candidate: &(&String, &Cell) = candidates.get(0).unwrap();
    let key = candidate.0;
    let cell = candidate.1;
    for value in cell.values.chars() {
        let mut field = initial_field.clone();
        match assign_value(&mut field, key, &value.to_string()) {
            Some(_) => {
                match search(&field) {
                    Some(f) => {
                        return Some(f)
                    }
                    None => ()
                }
            },
            None => ()
        }
    }
    return None;
}
