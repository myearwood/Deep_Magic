
// Private functions
// Implementation of square checking for both add and mult squares
// 

fn check_op(sq: &mut Vec<i32>, order: i32, op: &str) -> bool {
    // make sure we are counting unique integers
    sq.dedup();

    // Return false is the length is wrong
    let expected_sq_length = (order * order) as usize;  
    if sq.len() != expected_sq_length {
        return false;
    }

    let mut results: Vec<i32> = Vec::new();

    for i in 0..order {
        let row_result = get_row_op(sq, i, order, op);
        results.push(row_result);

        let col_result = get_col_op(sq, i, order, op);
        results.push(col_result);
    }

    let (c1_acc, c2_acc) = get_daigonals_op(sq, order, op);
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


// 
//  Public Functions
// 


pub fn add_magic(sq: &mut Vec<i32>, order: i32) -> bool {
    check_op(sq, order, "add")
}

// pub fn mult_magic(sq: &mut Vec<i32>, order: i32) -> bool {
//     check_op(sq, order, "mult")
// }