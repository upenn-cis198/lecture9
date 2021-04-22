/*
    Parallel and concurrent programming in Rust

    3 types of parallelism:
    - threads
    - processes
    - machines/clusters (distributed programming)

    Threads:
    std::thread::spawn
    std::thread::join
*/

const BIG_WORK: usize = 1_000_000_000;
const BIG_MOD: usize = 999983;

pub fn big_function() {
    let mut total = 0;
    println!("Starting busy loop");
    for i in 0..BIG_WORK {
        total = (total + i) % BIG_MOD;
    }
    println!("Total: {}", total);
}

// use std::thread;
// use std::sync::mpsc;

// pub fn big_function_par() {
//     let handle = thread::spawn(|| {
//     let result = handle.join().unwrap();
// }

/*
    The Send trait

    https://doc.rust-lang.org/nightly/core/marker/trait.Send.html
*/

/*
    The Sync trait

    https://doc.rust-lang.org/std/marker/trait.Sync.html
*/

/*
    Using processes instead of threads
*/

/*
    More high-level parallelism:

    Rayon
    https://docs.rs/rayon/1.5.0/rayon/
*/

use rayon::prelude::*;

pub fn sum_squares(n: usize) -> usize {
    (1..=n).map(|i| i * i).sum()
}

pub fn sum_squares_parallel(n: usize) -> usize {
    (1..=n).into_par_iter().map(|i| i * i).sum()
}

pub fn main() {
    println!("{}", sum_squares(1000000));
    println!("{}", sum_squares_parallel(1000000));
}
