
use rand::{self, Rng};

// Should refactor this into a common module
pub fn get_random_group(min: i32, max: i32) -> (Vec<i32>, Vec<i32>) {

    // thread_rng is often the most convenient source of randomness:
    let mut rng = rand::thread_rng();

    let mut g1: Vec<i32> = Vec::new();
    let mut g2: Vec<i32> = Vec::new();

    while g1.len() < 6 {
        let num: i32 = rng.gen_range(min, max);
        g1.push(num);
    }

    while g2.len() < 6 {
        let num: i32 = rng.gen_range(min, max);
        g2.push(num);
    }

    (g1, g2)
}

pub fn gen_sq(g1: &Vec<i32>, g2: &Vec<i32>) -> Vec<i32> {
    let mut v: Vec<i32> = Vec::new();
    let zero: i32 = 0;

    // Row 1
    v.push(g1[0] + g2[0]);
    v.push(g1[1] + g2[1]);
    v.push(g1[2] + g2[2]);
    v.push(g1[3] + g2[3]);
    v.push(g1[4] + g2[4]);
    v.push(g1[5] + g2[5]);



    // Row 2
    v.push(g1[2] + g2[3]);
    v.push(g1[3] + g2[4]);
    v.push(g1[4] + g2[5]);
    v.push(g1[5] + g2[0]);
    v.push(g1[0] + g2[1]);
    v.push(g1[1] + g2[2]);

    // Row 3
    v.push(g1[5] + g2[1]);
    v.push(g1[0] + g2[2]);
    v.push(g1[1] + g2[3]);
    v.push(g1[2] + g2[4]);
    v.push(g1[3] + g2[5]);
    v.push(g1[5] + g2[0]);


    // Row 4
    v.push(g1[4] + g2[2]);
    v.push(g1[5] + g2[3]);
    v.push(g1[0] + g2[4]);
    v.push(g1[1] + g2[5]);
    v.push(g1[2] + g2[0]);
    v.push(g1[3] + g2[1]);


    // Row 5
    v.push(g1[1] + g2[5]);
    v.push(g1[2] + g2[0]);
    v.push(g1[3] + g2[1]);
    v.push(g1[4] + g2[2]);
    v.push(g1[5] + g2[3]);
    v.push(g1[0] + g2[4]);


    // Row 2
    v.push(g1[3] + g2[4]);
    v.push(g1[4] + g2[5]);
    v.push(g1[5] + g2[0]);
    v.push(g1[0] + g2[1]);
    v.push(g1[1] + g2[2]);
    v.push(g1[2] + g2[3]);

    println!("{:?}", v);

    // Check that all the integers are unique.
    let mut sq_set = v.to_vec();
    sq_set.sort();
    sq_set.dedup();

    if sq_set.contains(&zero) {
        return vec![-1]
    }

    if sq_set.len() == 36 {
        v
    } else {
        vec![-1]
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sq_generator_for_6() {

        for _ in 0..50 {
            let (g1, g2) = get_random_group(0,400);
            let res = gen_sq(&g1, &g2);
            println!("{:?}", res.len());
        }

        assert_eq!(6, 36);
    }


}