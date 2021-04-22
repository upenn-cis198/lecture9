/*
    Popular crates:
    https://crates.io/crates?sort=recent-downloads
    (by downloads in the last 90 days)

    (By all-time downloads: https://crates.io/crates?sort=downloads)

    In the top 15 as of today:

    - cfg-if
    - rand, rand_core (and others)
    - libc
    - serde
    - log
    - syn, quote, other macro tools
    - lazy-static
    - bitflags

    Some we have already seen: serde, libc
*/

/*
    RNG
    https://docs.rs/rand/0.8.3/rand/

    Methods on thread_rng():
    https://docs.rs/rand/0.8.3/rand/trait.Rng.html#method.gen
*/

use rand::prelude::*;

pub fn rand_bools(_n: usize) -> Vec<usize> {
    // rand::thread_rng
    // TODO
    unimplemented!()
}

pub fn rand_digit() -> u8 {
    let result = 7;
    // TODO
    debug_assert!(result < 10);
    result
}

pub fn shuffle(v: &mut Vec<usize>) {
    v.shuffle(&mut rand::thread_rng());
}

/*
    Logging
    https://crates.io/crates/log
    https://crates.io/crates/env_logger
*/

// use log::{debug, info};
use std::error::Error;
use std::io::{self, Write};
use std::str::FromStr;

// TODO Task: add logging to the following function

// Generic utility function to get input from stdin
pub fn get_user_input<T>(msg: &str) -> Result<T, Box<dyn Error>>
where
    T: FromStr,
    <T as FromStr>::Err: Error + 'static,
{
    print!("{}: ", msg);
    io::stdout().flush()?;
    let mut raw = String::new();
    io::stdin().read_line(&mut raw)?;
    let trimmed = raw.trim();
    let val: T = trimmed.parse::<T>()?;
    Ok(val)
}

// Using this in a longer example: bin/log.rs

/*
    CFG annotations

    Utility for this:
    cfg_if
    https://docs.rs/cfg-if/1.0.0/cfg_if/
*/

// CFG annotations allow the compiler to compile something or not
// depending on some settings

// For those with familiarity with C: see macros like
// #IFDEBUG

#[cfg(test)]
fn test_from_int(n: usize) {
    //
}

#[test]
fn test_examples() {
    test_from_int(1);
    test_from_int(2);
    test_from_int(3);
}

// Debug vs non-debug implementations

#[cfg(not(debug_assertions))]
pub fn perf_critical_fun() {
    for n in 0..10 {
        println!("{}", n);
    }
}

#[cfg(debug_assertions)]
pub fn perf_critical_fun() {
    // TODO
}

pub fn perf_critical_fun_2() -> usize {
    let mut x = 0;
    for n in 0..10 {
        if cfg!(debug_assertions) {
            println!("n = {}", n);
        }
        x += n;
    }
    x
}

// Example from cfg_if docs:

use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(unix)] {
        pub fn foo() { /* unix specific functionality */ }
    } else if #[cfg(target_pointer_width = "32")] {
        pub fn foo() { /* non-unix, 32-bit functionality */ }
    } else {
        pub fn foo() { /* fallback implementation */ }
    }
}

/*
    A couple other interesting crates...
*/

/*
    Lazy Static
    https://docs.rs/lazy_static/1.4.0/lazy_static/
*/

// Problem we run into when defining constants:
const _MAX_CACHE_SIZE_1: usize = 1000; // replaced as const by compiler
static _MAX_CACHE_SIZE_2: usize = 1000; // stored in binary as memory location
static mut _MAX_CACHE_SIZE_3: usize = 1000; // modifying is unsafe but allowed

const _SMALL_PRIMES_1: &[usize] = &[2, 3, 5, 7, 11, 13, 17, 19];

// What if we want a dynamically allocated constant or static variable?
// const SMALL_PRIMES_2: Vec<usize> = vec![2, 3, 5, 7, 11, 13, 17, 19];
// Const function: doesn't work either
// const fn small_primes() -> Vec<usize> {
//     vec![2, 3, 5, 7, 11, 13, 17, 19]
// }

use lazy_static::lazy_static;

lazy_static! {
    static ref SMALL_PRIMES_3: Vec<usize> = vec![2, 3, 5, 7, 11, 13, 17, 19];
}

/*
    Bitflags
    https://docs.rs/bitflags/1.2.1/bitflags/

    See example there.
*/
