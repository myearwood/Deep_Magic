use acc;

//
// Implementation of square checking for both add and mult squares
// 

fn check_op(sq: &Vec<i32>, order: i32, op: &str) -> bool {
    // Return false is the length is wrong
    let expected_sq_length = (order * order) as usize;  
    if sq.len() != expected_sq_length {
        return false;
    }

    let mut results: Vec<i64> = Vec::new();

    for i in 0..order {
        let row_result = acc::get_row_op(sq, i, order, op);
        results.push(row_result);

        let col_result = acc::get_col_op(sq, i, order, op);
        results.push(col_result);
    }

    let (c1_acc, c2_acc) = acc::get_daigonals_op(sq, order, op);
    results.push(c1_acc);
    results.push(c2_acc);

    // verify we calculated correct number of sums 
    let expected_results_length = (2*order + 2) as usize;
    if results.len() != expected_results_length {
        panic!("wrong number of results computed!")
    }

    // remove duplicates
    results.dedup();
    
    if results.len() != 1 {
        return false
    } else {
        return true
    }
}

// 
//  Public Functions
// 


pub fn add_magic(sq: &Vec<i32>, order: i32) -> bool {
    check_op(sq, order, "add")
}

pub fn mult_magic(sq: &Vec<i32>, order: i32) -> bool {
    check_op(sq, order, "mult")
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
    fn test_correct_add_sq() {
        let filename = "data/correct_sq.txt";
        let sq = etl::read_sq(filename);
        let valid_add_magic = add_magic(&sq, 5);
        assert_eq!(valid_add_magic, true);
    }

    #[test]
    fn test_bad_sum_sq() {
        let filename = "data/bad_sum_sq.txt";
        let sq = etl::read_sq(filename);
        let valid_add_magic = add_magic(&sq, 5);
        assert_eq!(valid_add_magic, false);
    }

    #[test]
    fn test_malformed_sq() {
        let filename = "data/malformed_sq.txt";
        let sq = etl::read_sq(filename);
        let valid_add_magic = add_magic(&sq, 5);
        assert_eq!(valid_add_magic, false);
    }

    #[test]
    fn test_semi_magic_sq() {
        let filename = "data/semi_magic_sq.txt";
        let sq = etl::read_sq(filename);
        let valid_add_magic = add_magic(&sq, 5);
        assert_eq!(valid_add_magic, false);        
    }


    //
    //  Tests for mult magic squares
    //

    #[test]
    fn test_correct_mult_sq() {
        let filename = "data/mult_magic_sq.txt";
        let sq = etl::read_sq(filename);
        let valid_mult_magic = mult_magic(&sq, 5);
        assert_eq!(valid_mult_magic, true);
    }


    #[test]
    fn test_mult_semi_magic_sq() {
        let filename = "data/mult_semi_magic_sq.txt";
        let sq = etl::read_sq(filename);
        let valid_mult_magic = mult_magic(&sq, 5);
        assert_eq!(valid_mult_magic, false);
    }

}




