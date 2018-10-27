mod checker;
mod etl;
mod acc;
mod sample;
mod enigma_6;
extern crate rand;
use std::sync::{Mutex, Arc};
use std::thread;


fn gen_sample() {
    let (g1, g2) = enigma_6::get_random_group(-10,200);
    let rand_sq = enigma_6::gen_sq(g1, g2);

    let _sums_correct = sample::add_magic(&rand_sq, 5);
    let _products_correct = sample::mult_magic(&rand_sq, 5);
    // println!("Square: {:?}", rand_sq);
    // println!("Sums Correct: {:?}", sums_correct);
    // println!("Products Correct: {:?}", products_correct);   
}


#[allow(dead_code)]
fn check_generated() {
    let (g1, g2) = enigma_6::get_random_group(-10,200);
    let rand_sq = enigma_6::gen_sq(g1, g2);
    let _is_add_magic = checker::add_magic(&rand_sq, 5);
    let _is_mult_magic = checker::mult_magic(&rand_sq, 5);
}

fn main() {
    // let base: i64 = 2;
    // let stopping_point = base.pow(32);
    let num_threads: usize = 5;
    let array: [i64; 13] = [0; 13];
    let counter = Arc::new(Mutex::new(array));
    let mut handles = vec![];

    for _ in 0..num_threads {
        let counter = Arc::clone(&counter);

        // specifiy thread
        let handle = thread::spawn(move || {
            let interval = 25_000;
            let mut count = 0;

            loop {
                gen_sample();
                count += 1;

                if count % interval == 0 {
                    // update counter using mutex
                    let mut data_array = counter.lock().unwrap();
                    data_array[12] += count;
                    println!("Current Count: {}", data_array[12]);
                    count = 0;                
                }
            }


        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}