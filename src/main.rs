mod checker;
mod etl;
mod acc;
mod sample;
mod enigma_6;
extern crate rand;
use std::time::Instant;
use std::time::Duration;
// use std::sync::{Mutex, Arc};
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

    let all_threads_start = Instant::now();


    // let base: i64 = 2;
    // let stopping_point = base.pow(32);
    let num_threads: usize = 5;
    // let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..num_threads {
        // let counter = Arc::clone(&counter);

        // specifiy thread
        let handle = thread::spawn(move || {
            // try it without mutex first
            // let mut num = counter.lock().unwrap();
            let one_min = Duration::new(60, 0);
            let start = Instant::now();
            let mut elapsed = start.elapsed();
            let mut count = 0;

            while elapsed < one_min {
                gen_sample();
                count += 1;
                elapsed = start.elapsed();
            }

            println!("My thread 1 min count: {}", count)
            
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let elapsed = all_threads_start.elapsed();
    println!("Total time elapsed: {}", elapsed.as_secs());
}