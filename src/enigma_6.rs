use DATA_ARRAY_LEN;
use rand::{self, Rng};
use sample;
use permutohedron::Heap;


fn gen_all_perms (g1: &mut Vec<i32>, g2: &mut Vec<i32>) -> Vec<(Vec<i32>, Vec<i32>)> {
    let g1_perms = Heap::new(g1);
    let g2_perms = Heap::new(g2);
    let mut g1_vec: Vec<Vec<i32>> = Vec::new();
    let mut g2_vec: Vec<Vec<i32>> =Vec::new(); 
    let mut results: Vec<(Vec<i32>, Vec<i32>)> = Vec::new();

    for g1_p in g1_perms {
        g1_vec.push(g1_p.clone())
    }

    for g2_p in g2_perms {
        g2_vec.push(g2_p.clone())
    }

    for g1_p in &g1_vec {
        for g2_p in &g2_vec {
            results.push((g1_p.to_vec(), g2_p.to_vec()));
        }
    }

    results
}


pub fn get_random_group(min: i32, max: i32) -> (Vec<i32>, Vec<i32>) {

    // thread_rng is often the most convenient source of randomness:
    let mut rng = rand::thread_rng();

    let mut g1: Vec<i32> = Vec::new();
    let mut g2: Vec<i32> = Vec::new();

    while g1.len() < 5 {
        let num: i32 = rng.gen_range(min, max);
        g1.push(num);
    }

    while g2.len() < 5 {
        let num: i32 = rng.gen_range(min, max);
        g2.push(num);
    }

    (g1, g2)
}

pub fn gen_sq(g1: &Vec<i32>, g2: &Vec<i32>) -> Vec<i32> {
    let mut v: Vec<i32> = Vec::new();

    // Row 1
    v.push(g1[0] + g2[0]);
    v.push(g1[1] + g2[1]);
    v.push(g1[2] + g2[2]);
    v.push(g1[3] + g2[3]);
    v.push(g1[4] + g2[4]);


    // Row 2
    v.push(g1[2] + g2[3]);
    v.push(g1[3] + g2[4]);
    v.push(g1[4] + g2[0]);
    v.push(g1[0] + g2[1]);
    v.push(g1[1] + g2[2]);

    // Row 3
    v.push(g1[4] + g2[1]);
    v.push(g1[0] + g2[2]);
    v.push(g1[1] + g2[3]);
    v.push(g1[2] + g2[4]);
    v.push(g1[3] + g2[0]);

    // Row 4
    v.push(g1[1] + g2[4]);
    v.push(g1[2] + g2[0]);
    v.push(g1[3] + g2[1]);
    v.push(g1[4] + g2[2]);
    v.push(g1[0] + g2[3]);

    // Row 5
    v.push(g1[3] + g2[2]);
    v.push(g1[4] + g2[3]);
    v.push(g1[0] + g2[4]);
    v.push(g1[1] + g2[0]);
    v.push(g1[2] + g2[1]);

    // Check that all the integers are unique.
    let mut sq_set = v.to_vec();
    sq_set.sort();
    sq_set.dedup();

    if sq_set.len() == 25 {
        v
    } else {
        vec![-1]
    }
}



pub fn bulk_generate(count: &mut i64, results: &mut [i64; DATA_ARRAY_LEN]) {
    let (g1, g2) = get_random_group(-10,500);

    let rand_sq = gen_sq(&g1, &g2);

    let sums_count = sample::add_magic(&rand_sq, 5);
    let mult_count = sample::mult_magic(&rand_sq, 5);

    // if its not a add magic square, put it into the 0 index box
    if sums_count != 12 {
        results[0] += 1;
    } else { // add it to appropriate index 
        results[mult_count as usize] += 1;

        if mult_count == 4 {
            println!("4 sq: {:?}. {:?}", g1, g2);
        }
    }

    *count += 1;
}


//
// Unit Tests
// 

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gl_square_generator() {

        let g1 = vec![0, 76, 30, -23, -30];
        let g2 = vec![95, 100, 78, 114, 80];

        let gen_sq = gen_sq(&g1, &g2);

        let expected_result = vec![95, 176, 108, 91, 50, 144, 57, 65, 100, 154, 70, 78, 190, 110, 72, 156, 125, 77, 48, 114, 55, 84, 80, 171, 130];
        assert_eq!(gen_sq, expected_result);
    }

    #[test]
    fn test_get_random_group() {
        let (g1, g2) = get_random_group(0, 200);
        assert_eq!(g1.len(), 5);
        assert_eq!(g2.len(), 5);
    }

    #[test]
    fn test_square_generator_dedup() {
        let g1 = vec![0, 0, 0, 0, 0];
        let g2 = vec![0, 0, 0, 0, 1];

        let gen_sq = gen_sq(&g1, &g2);
        let expected_result = vec![-1];
        assert_eq!(gen_sq, expected_result);
    }

    #[test]
    fn test_gen_all_perms() {
        let mut g1 = vec![1, 2, 3, 4, 5];
        let mut g2 = vec![6, 7, 8, 9, 10];

        let all_combos = gen_all_perms(&mut g1, &mut g2);
        assert_eq!(all_combos.len(), 14400);
    }

    #[test]
    fn expirement_with_perms() {
        let mut results: [i64; DATA_ARRAY_LEN] = [0; DATA_ARRAY_LEN];
        let mut g1 = vec![480, 1, 325, 417, 105];
        let mut g2 = vec![238, 71, 453, -1, 58];

        let all_perms = gen_all_perms(&mut g1, &mut g2);

        for (g1_p, g2_p) in &all_perms {
            let rand_sq = gen_sq(&g1_p, &g2_p);

            let sums_count = sample::add_magic(&rand_sq, 5);
            let mult_count = sample::mult_magic(&rand_sq, 5);

            // if its not a add magic square, put it into the 0 index box
            if sums_count != 12 {
                results[0] += 1;
            } else { // add it to appropriate index 
                results[mult_count as usize] += 1;
            }
        }

        println!("Results: {:?}", results);
        assert_eq!(1, 8);
    }

    // #[test]
    // fn expirement_with_bulk_generate() {

    //     let mut count = 0 as i64;
    //     let mut array: [i64; DATA_ARRAY_LEN] = [0; DATA_ARRAY_LEN];


    //     for _ in 0..10_000_000 {
    //         bulk_generate(&mut count, &mut array);
    //     }

    //     assert_eq!(1, 1);
    // }
}


