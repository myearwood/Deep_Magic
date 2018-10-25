use std::fs::File;
use std::io::prelude::*;

fn convert_file_to_sq(contents: String) -> Vec<i32> {
    let mut v: Vec<i32> = Vec::new();
    let nums = contents.split_whitespace();

    for n_str in nums {
        let n: i32 = n_str.parse().unwrap();
        v.push(n);
    }
    v.dedup();
    v
}

pub fn read_sq(filename: &str) -> Vec<i32> {
    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();

    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    convert_file_to_sq(contents)
}