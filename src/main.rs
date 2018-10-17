use std::fs::File;
use std::io::prelude::*;


fn get_daigonals_op(sq: &Vec<i32>, order: i32, op: &str) -> (i32, i32) {
    let mut corner1: usize = 0;
    let mut corner2: usize = (order -1) as usize;

    let mut c1_accumulator: i32 = 0;
    let mut c2_accumulator: i32 = 0;

    // get sum for corner 1 (northwest corner)
    // next square = order + 1 from current
    for _ in  0..order{

        match op {
            "add" => c1_accumulator += sq[corner1],
            "mult" => c1_accumulator = c1_accumulator * sq[corner1],
            _ => panic!("op not found!"),
        }    

        corner1 += (order + 1) as usize;;
    }

    // get sum for corner 2 (northeast corner)
    // next square = order -1 from current
    for _ in 0..order{

        match op {
            "add" => c2_accumulator += sq[corner2],
            "mult" => c2_accumulator = c2_accumulator * sq[corner2],
            _ => panic!("op not found!"),
        }    

        corner2 += (order - 1) as usize;
    }

    (c1_accumulator, c2_accumulator)
}


fn get_row_op(sq: &Vec<i32>, i: i32, order: i32, op: &str) -> i32 {
    // Starting position: is i * order
    // Increment by 1


    let mut accumulator: i32 = 0;
    let start_pos: usize = (i * order) as usize;
    let end_pos: usize = start_pos + (order as usize);

    for x in start_pos..end_pos {
        match op {
            "add" => accumulator += sq[x],
            "mult" => accumulator = accumulator * sq[x],
            _ => panic!("op not found!"),
        }
    }

    accumulator
}


fn get_col_op(sq: &Vec<i32>, i: i32, order: i32, op: &str) -> i32 {
    // Starting positon is i
    // Increment by order

    let mut accumulator: i32 = 0;
    let mut start_pos: i32 = i;
    for _ in 0..order {
        match op {
            "add" => accumulator += sq[start_pos as usize],
            "mult" => accumulator = accumulator * sq[start_pos as usize],
            _ => panic!("op not found!"),
        }

        start_pos += order       
    }
    accumulator



}

fn check_add_magic(sq: &mut Vec<i32>, order: i32) -> bool {
    // make sure we are counting unique integers
    sq.dedup();

    // Return false is the length is wrong
    let expected_sq_length = (order * order) as usize;  
    if sq.len() != expected_sq_length {
        return false;
    }

    let mut sums: Vec<i32> = Vec::new();

    for i in 0..order {
        let row_sum = get_row_op(sq, i, order, "add");
        sums.push(row_sum);

        let col_sum = get_col_op(sq, i, order, "add");
        sums.push(col_sum);
    }

    let (c1_acc, c2_acc) = get_daigonals_op(sq, order, "add");
    sums.push(c1_acc);
    sums.push(c2_acc);

    // verify we calculated correct number of sums 
    let expected_sums_length = (2*order + 2) as usize;
    if sums.len() != expected_sums_length {
        panic!("wrong number of sums computed!")
    }

    // remove duplicates
    sums.dedup();
    
    if sums.len() != 1 {
        return false
    } else {
        return true
    }
}

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
    let valid_add_magic = check_add_magic(&mut sq, 5);
    println!("Is the square valid: {}", valid_add_magic)
}