use acc;
use std::collections::HashMap;


// Nb: we are making less checks about the validity of these squares.
// We are asummeing we generated them so they are correct./ 
//
// Relaxed Assumptions vs Check
// 
// No malformed checks

fn add_to_map(results: &mut HashMap<i64, i32>, acc: i64) {
    if !results.contains_key(&acc) {
        results.insert(
            acc,
            1
        );
    } else {
        let current_sum: i32 = *results.get(&acc).unwrap();
        results.insert(
            acc,
            current_sum + 1,
        );
    }
}

fn highest_common_sum(results: HashMap<i64, i32>) -> i32 {
    let mut max_fequency: i32 = 0;

    for (_sum, freq) in results.iter() {
        if freq > &max_fequency {
            max_fequency = *freq 
        } 
    }

    max_fequency
}


//
// Implementation of square checking for both add and mult squares
// 

fn sample_op(sq: &Vec<i32>, order: i32, op: &str) -> i32 {

    // Return false is the length is wrong
    let expected_sq_length = (order * order) as usize;  
    if sq.len() != expected_sq_length {
        return -1;
    }

    let mut results = HashMap::new();

    for i in 0..order {
        let row_result = acc::get_row_op(sq, i, order, op);
        add_to_map(&mut results, row_result);

        let col_result = acc::get_col_op(sq, i, order, op);
        add_to_map(&mut results, col_result);
    }

    let (c1_acc, c2_acc) = acc::get_daigonals_op(sq, order, op);
    add_to_map(&mut results, c1_acc);
    add_to_map(&mut results, c2_acc);

    highest_common_sum(results)
}

// 
//  Public Functions
// 


pub fn add_magic(sq: &Vec<i32>, order: i32) -> i32 {
    sample_op(sq, order, "add")
}

pub fn mult_magic(sq: &Vec<i32>, order: i32) -> i32 {
    sample_op(sq, order, "mult")
}


//
// Unit Tests
// 

#[cfg(test)]
mod tests {
    use super::*;
    use etl;


    //
    //  Tests for add magic squares
    //

    #[test]
    fn sample_correct_add_sq() {
        let filename = "data/correct_sq.txt";
        let sq = etl::read_sq(filename);
        let add_magic_sums = add_magic(&sq, 5);
        assert_eq!(add_magic_sums, 12);
    }

    #[test]
    fn test_semi_magic_sq() {
        let filename = "data/semi_magic_sq.txt";
        let sq = etl::read_sq(filename);
        let add_magic_sums = add_magic(&sq, 5);
        assert_eq!(add_magic_sums, 10);        
    }

    #[test]
    fn test_bad_sum_sq() {
        let filename = "data/bad_sum_sq.txt";
        let sq = etl::read_sq(filename);
        let add_magic_sums = add_magic(&sq, 5);
        assert_eq!(add_magic_sums, 8);
    }

    //
    // Test for Mult Magic Squares
    //

    #[test]
    fn test_correct_mult_sq() {
        let filename = "data/mult_magic_sq.txt";
        let sq = etl::read_sq(filename);
        let mult_magic_sums = mult_magic(&sq, 5);
        assert_eq!(mult_magic_sums, 12);
    }


    #[test]
    fn test_mult_semi_magic_sq() {
        let filename = "data/mult_semi_magic_sq.txt";
        let sq = etl::read_sq(filename);
        let mult_magic_sums = mult_magic(&sq, 5);
        assert_eq!(mult_magic_sums, 11);
    }

}