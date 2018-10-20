//
// Accumulator Module
// used to do the grunt work for the checker and sampling code
//

pub fn get_daigonals_op(sq: &Vec<i32>, order: i32, op: &str) -> (i32, i32) {
    let mut corner1: usize = 0;
    let mut corner2: usize = (order -1) as usize;

    let mut c1_accumulator: i32 =  match op {
        "add" => 0,
        "mult" => 1,
        _ => panic!("op not found!"),
    };


    let mut c2_accumulator: i32 =  match op {
        "add" => 0,
        "mult" => 1,
        _ => panic!("op not found!"),
    };

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


pub fn get_row_op(sq: &Vec<i32>, i: i32, order: i32, op: &str) -> i32 {
    // Starting position: is i * order
    // Increment by 1


    let mut accumulator: i32 = match op {
        "add" => 0,
        "mult" => 1,
        _ => panic!("op not found!"),
    };

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


pub fn get_col_op(sq: &Vec<i32>, i: i32, order: i32, op: &str) -> i32 {
    // Starting positon is i
    // Increment by order

    let mut accumulator: i32 =  match op {
        "add" => 0,
        "mult" => 1,
        _ => panic!("op not found!"),
    };

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