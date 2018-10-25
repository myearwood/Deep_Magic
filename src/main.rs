mod checker;
mod etl;
mod acc;
mod sample;
mod enigma_6;
extern crate rand;

fn main() {
    let (g1, g2) = enigma_6::get_random_group(-10,200);
    let mut rand_sq = enigma_6::gen_sq(g1, g2);

    let sums_correct = sample::add_magic(&mut rand_sq, 5);
    let products_correct = sample::mult_magic(&mut rand_sq, 5);
    println!("Square: {:?}", rand_sq);
    println!("Sums Correct: {:?}", sums_correct);
    println!("Products Correct: {:?}", products_correct);

}