use lecture9::popular;

fn main() {
    println!("CIS198 Lecture 9: Useful Crates");

    /* RNG */
    let mut v = popular::rand_bools(5);
    println!("Random bools: {:?}", v);
    popular::shuffle(&mut v);
    println!("Shuffled: {:?}", v);
    println!(
        "Some random digits: {}{}{}{}{}",
        popular::rand_digit(),
        popular::rand_digit(),
        popular::rand_digit(),
        popular::rand_digit(),
        popular::rand_digit(),
    );

    /* Logging */
    // Initialize the logger
    env_logger::init();
    // By default logging happens at warn level
    let n: isize = popular::get_user_input("Input a number").expect("error!");
    println!("Parsed number: {}", n);
}
