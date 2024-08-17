use std::fs::read_to_string;

pub fn read_input(path: &str) -> String {
    read_to_string(path)
        .expect("Input file could not be opened. Please ensure input.txt is in the root directory!")
}
