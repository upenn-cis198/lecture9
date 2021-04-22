use lecture9::popular::get_user_input;
use log::{debug, error, info, log_enabled, warn, Level};

fn main() {
    env_logger::init();

    info!("Getting user input");
    let n = get_user_input::<isize>("Type an integer").unwrap_or_else(|err| {
        error!("Fatal: get user input failed: {}", err);
        panic!();
    });

    if n > 1000 {
        warn!("User input was very large!")
    }

    if log_enabled!(Level::Debug) {
        for i in 0..n {
            debug!("{}", i);
        }
    }

    println!("User input: {}", n);

    info!("Done");
}
