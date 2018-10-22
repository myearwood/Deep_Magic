
fn generate_square(g1: Vec<i32>, g2: Vec<i32>) -> Vec<i32> {
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

    v
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

        let gen_sq = generate_square(g1, g2);

        let expected_result = vec![95, 176, 108, 91, 50, 144, 57, 65, 100, 154, 70, 78, 190, 110, 72, 156, 125, 77, 48, 114, 55, 84, 80, 171, 130];
        assert_eq!(gen_sq, expected_result);
    }


}


