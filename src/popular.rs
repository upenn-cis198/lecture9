/*
    Popular crates:
    https://crates.io/crates?sort=recent-downloads
    (by downloads in the last 90 days)

    (By all-time downloads: https://crates.io/crates?sort=downloads)

    In the top 15-20 as of today:

    - cfg-if
    - rand, rand_core (and others)
    - log
    - syn, quote, other macro tools
    - lazy-static
    - bitflags

    Some we have already seen: serde, libc, regex, structopt
*/

/*
    RNG
    https://docs.rs/rand/0.8.3/rand/

    Rust doesn't have RNG in the standard library
    - Sometimes Rust tries to keep std minimal in some respects

    Methods on thread_rng():
    https://docs.rs/rand/0.8.3/rand/trait.Rng.html#method.gen
*/

use rand::prelude::*;

pub fn rand_bools(n: usize) -> Vec<bool> {
    // To use rand, call rand::thread_rng()
    // Random number generator specific to this thread
    // (internally it has to be seeded initially somehow, based on the thread)

    let mut result = Vec::new();
    for _ in 0..n {
        // Flip a coin
        let b: bool = rand::thread_rng().gen();
        result.push(b);
    }

    result
}

// .gen() function generates a random value of a type
// (will work for random bool, random usize, random u8, ...)
// For random in a certain range:

pub fn rand_digit() -> u8 {
    let result = thread_rng().gen_range(0..10);
    debug_assert!(result < 10);
    result
}

pub fn shuffle<T>(v: &mut Vec<T>) {
    v.shuffle(&mut rand::thread_rng());
}

/*
    Logging
    https://crates.io/crates/log
    https://crates.io/crates/env_logger

    If you're used to for example Python, you may have seen loging
    logger::get_logger();
    logger.warn(...)

    Logging in Rust is divided into two pieces.
    - Logging *facade* which gives an interface for logging
      (allows you to call the logger, with info, debug, warn, error)
    - Logging *implementation* which actually implements the logging

    You might log to different targets:
    - log msgs to stdout
    - log msgs to a database
    - log msgs to a file

    interface: log
    implementation: env_logger
*/

use log::{debug, error, info, warn};
use std::error::Error;
use std::io::{self, Write};
use std::str::FromStr;

// TODO Task: add logging to the following function

// Generic utility function to get input from stdin
pub fn get_user_input<T>(msg: &str) -> Result<T, Box<dyn Error>>
where
    T: FromStr, // allows us to .parse::<T>()
    <T as FromStr>::Err: Error + 'static,
{
    info!("getting user input ({})", msg);

    debug!("printing console message to user");
    print!("{}: ", msg);
    // flush stdout to make sure it shows up
    io::stdout().flush().map_err(|err| {
        error!("Not able to flush stdout: {}", err);
        err
    })?;

    debug!("getting user input from stdin");
    let mut raw = String::new();
    io::stdin().read_line(&mut raw).map_err(|err| {
        error!("Not able to read line from stdin: {}", err);
        err
    })?; // propagating error

    debug!("trimming user input");
    let trimmed = raw.trim();

    debug!("Parsing input as type T");
    let val: T = trimmed.parse::<T>().map_err(|err| {
        warn!("Not able to parse user input as int: {} ({})", trimmed, err);
        err
    })?;

    debug!("returning from get_user_input function");
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
// #IFDEF UNITTESTS

// When writing tests, sometimes you want a function that's only compiled in test mode.
// Functions that are only used for testing

pub fn double(n: isize) -> isize {
    n * 2
}

// CFG annotation: only compile this in a certain mode
#[cfg(test)]
fn test_double_of(n: isize) {
    assert_eq!(double(n), n + n);
}

#[test]
fn test_examples() {
    test_double_of(1);
    test_double_of(2);
    test_double_of(3);
    test_double_of(4);
    test_double_of(5);
    test_double_of(6);
    test_double_of(7);
    test_double_of(8);
}

// Debug vs non-debug implementations

#[cfg(not(debug_assertions))]
pub fn perf_critical_fun() {
    let mut x = 0;
    for n in 0..1000000 {
        x += n;
    }
    println!("Result: {}", x);
}

#[cfg(debug_assertions)]
#[allow(clippy::explicit_counter_loop)]
pub fn perf_critical_fun() {
    let mut x = 0;
    let mut count_loops = 0;
    for n in 0..1000000 {
        x += n;
        assert_eq!(n, count_loops);
        assert_eq!(x, n * (n + 1) / 2);
        count_loops += 1;
    }
    println!("Result: {}", x);
}

// For example: tracking cache hits/cache misses in data structure

pub fn perf_critical_fun_2() -> usize {
    let mut x = 0;
    for n in 0..10 {
        // Syntax for inside an if statement
        if cfg!(debug_assertions) {
            println!("n = {}", n);
        }
        x += n;
    }
    x
}

// cfg_if: syntactic support for cfg annotations / checks
// (you can use the cfg annotations even without cfg_if,
// just makes it easier if checking lots of them or if
// you have a large if block.)

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

    Problem: when defining constants in our code,
    Rust only lets us define constants that are known at compile time.
    Inconvenient we want heap-allocated, complex constants
*/

// Problem we run into when defining constants:
const _MAX_CACHE_SIZE_1: usize = 10_000;
// ^ replaced as const by compiler
static _MAX_CACHE_SIZE_2: usize = 10_000;
// ^ stored in binary as memory location
static mut _MAX_CACHE_SIZE_3: usize = 10_000;
// ^ modifying is unsafe but allowed
// (like a global variable -- probably not usually a good idea)

// const and static above are the same in behavior, though
// implemented differently and const should be faster.

const _TMP_FILE_LOC: &str = "tmp/output.tmp";

const _SMALL_PRIMES_1: &[usize] = &[2, 3, 5, 7, 11, 13, 17, 19];

// Regardless of which of these I choose, though, I can't do constants
// of all types.
// In particular, what if we want a dynamically allocated constant or
// static variable?

// const SMALL_PRIMES_2: Vec<usize> = vec![2, 3, 5, 7, 11, 13, 17, 19];

// Example const fn:
pub const fn calculate_big_number() -> usize {
    // Executes code at compile time!
    143 * 143 * 143 * 143
}

// Const function: doesn't work either
// const fn small_primes() -> Vec<usize> {
//     // vec! not allowed at compile time.
//     vec![2, 3, 5, 7, 11, 13, 17, 19]
// }

use lazy_static::lazy_static;

// "static" variables: dynamic persistent variables
// for example: write a memoized function that remembers things across calls
// or, e.g. a function like a prime generator which remembers the previous
// primes generated and only generates if new ones are needed

lazy_static! {
    // can ignore ref, it's just required lazy_static syntax
    static ref SMALL_PRIMES_3: Vec<usize> = vec![2, 3, 5, 7, 11, 13, 17, 19];

    // Internally, the vector is NOT initialized until the first time its used.
    // after it's used once, future references point to the same result.
    // or could even be mutable.
}

/*
    Bitflags
    https://docs.rs/bitflags/1.2.1/bitflags/

    See example there.
*/
