// mod checker;
// mod etl;
mod acc;
mod sample;
mod enigma_6;
extern crate rand;
use std::time::Instant;
use std::time::Duration;

fn gen_sample() {
    let (g1, g2) = enigma_6::get_random_group(-10,200);
    let mut rand_sq = enigma_6::gen_sq(g1, g2);

    let _sums_correct = sample::add_magic(&mut rand_sq, 5);
    let _products_correct = sample::mult_magic(&mut rand_sq, 5);
    // println!("Square: {:?}", rand_sq);
    // println!("Sums Correct: {:?}", sums_correct);
    // println!("Products Correct: {:?}", products_correct);   
}

fn main() {
    let one_min = Duration::new(60, 0);
    let start = Instant::now();
    let mut elapsed = start.elapsed();
    let mut count = 0;

    while elapsed < one_min {
        gen_sample();
        count += 1;
        elapsed = start.elapsed();
    }

    println!("{} squares generated per minute.", count);

}