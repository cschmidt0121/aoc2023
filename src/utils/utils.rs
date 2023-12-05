use std::fs::read_to_string;

// Taken from rust docs https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
// I didn't use the more efficient iterator version as AOC input is never that big
pub fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}