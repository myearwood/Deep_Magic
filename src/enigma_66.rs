

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
    v.push(g1[4] + g2[0]);
    v.push(g1[5] + g2[1]);
    v.push(g1[0] + g2[2]);
    v.push(g1[1] + g2[3]);
    v.push(g1[2] + g2[4]);
    v.push(g1[3] + g2[5]);


    // Row 4
    v.push(g1[1] + g2[4]);
    v.push(g1[2] + g2[5]);
    v.push(g1[3] + g2[0]);
    v.push(g1[4] + g2[1]);
    v.push(g1[5] + g2[2]);
    v.push(g1[0] + g2[3]);


    // Row 5
    v.push(g1[3] + g2[1]);
    v.push(g1[4] + g2[2]);
    v.push(g1[5] + g2[3]);
    v.push(g1[0] + g2[4]);
    v.push(g1[1] + g2[5]);
    v.push(g1[2] + g2[0]);


    // Row 2
    v.push(g1[5] + g2[4]);
    v.push(g1[0] + g2[5]);
    v.push(g1[1] + g2[0]);
    v.push(g1[2] + g2[1]);
    v.push(g1[3] + g2[2]);
    v.push(g1[4] + g2[3]);

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