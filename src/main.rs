use std::fs::File;
use std::io::prelude::*;
mod checker;

fn convert_file_to_sq(contents: String) -> Vec<i32> {
    let mut v: Vec<i32> = Vec::new();
    let nums = contents.split_whitespace();

    for n_str in nums {
        let n: i32 = n_str.parse().unwrap();
        v.push(n);
    }
    v
}

fn read_square_from_file(filename: &str) -> Vec<i32> {
    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();

    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    convert_file_to_sq(contents)
}

fn main() {

    let filename = "data/correct_sq.txt";
    let mut sq = read_square_from_file(filename);
    let valid_add_magic = checker::add_magic(&mut sq, 5);
    println!("Is the square valid: {}", valid_add_magic)
}