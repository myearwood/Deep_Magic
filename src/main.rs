mod checker;
mod etl;
mod acc;
mod sample;
mod enigma_6;
mod enigma_66;
use std::fs;

extern crate rand;
extern crate permutohedron;
use std::sync::{Mutex, Arc};
use std::{thread, time};


const DATA_ARRAY_LEN: usize = 14;


#[allow(dead_code)]
fn check_generated() {
    let (g1, g2) = enigma_6::get_random_group(-10,200);
    let rand_sq = enigma_6::gen_sq(&g1, &g2);
    let _is_add_magic = checker::add_magic(&rand_sq, 5);
    let _is_mult_magic = checker::mult_magic(&rand_sq, 5);
}

fn main() {
    // let base: i64 = 2;
    // let stopping_point = base.pow(32);
    let num_threads: usize = 5;


    // initialize the global array
    let mut array: Vec<(i64, Vec<Vec<i32>>)> = Vec::new();
    for _ in 0..DATA_ARRAY_LEN {
        array.push((0,Vec::new()));
    }


    let array_ref = Arc::new(Mutex::new(array));
    let mut handles = vec![];

    // initialize processing threads 
    for _ in 0..num_threads {
        let array_ref = Arc::clone(&array_ref);

        // specifiy thread
        let handle = thread::spawn(move || {
            let interval = 25_000 as i64;
            let mut local_count = 0 as i64;

            // initialize the local array
            let mut local_array: Vec<(i64, Vec<Vec<i32>>)> = Vec::new();
            for _ in 0..DATA_ARRAY_LEN {
                local_array.push((0,Vec::new()));
            }

            loop {
                enigma_6::bulk_generate(&mut local_count, &mut local_array);

                if local_count % interval == 0 {
                    // update counter using mutex
                    let mut data_array = array_ref.lock().unwrap();

                    for i in 0..DATA_ARRAY_LEN {
                        data_array[i].0 += local_array[i].0;


                        // If we need more examples and this thread has exmaples,
                        // add them to global
                        let global_examples = data_array[i].1.len();
                        let local_examples = local_array[i].1.len();

                        if global_examples < 10 && local_examples > 0 {
                            let diff = 10 - global_examples;
                            let lower_bound = if diff > local_examples { local_examples } else { diff };

                            for x in 0..lower_bound {
                                let copied_example = local_array[i].1[x].clone();
                                data_array[i].1.push(copied_example);
                            }

                            // reset local array so we dont get repeated examples
                            local_array[i].1 = Vec::new();
                        }
                    }

                    data_array[13].0 += interval;
                    local_count = 0;
                }
            }


        });
        handles.push(handle);
    }


    // initialize the write to disk thread
    let array_ref = Arc::clone(&array_ref);
    let io_handle = thread::spawn(move || {
        loop {
            let one_min = time::Duration::new(60, 0);
 
            thread::sleep(one_min);

            // write to file
            let mut data_string = "".to_owned();
            let data_array = array_ref.lock().unwrap();

            for i in 1..DATA_ARRAY_LEN-1 {
                let next_num = format!("{} magic product: {}\n", i, data_array[i].0.to_string());
                data_string.push_str(&next_num);

                //write the examples

                let example_len = data_array[i].1.len();
                for x in 0..example_len {
                    let example = format!("{:?}\n", data_array[i].1[x]);
                    data_string.push_str(&example);
                }
            }

            let total_records = format!("{} total_records.", data_array[DATA_ARRAY_LEN-1].0);
            data_string.push_str(&total_records);

            fs::write("results.txt", data_string).expect("Unable to write file");
            println!("Total Records: {}", data_array[DATA_ARRAY_LEN-1].0);
        }
    });
    handles.push(io_handle);

    for handle in handles {
        handle.join().unwrap();
    }
}