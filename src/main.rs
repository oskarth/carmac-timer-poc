use std::thread::sleep;
use std::time::{Duration, Instant};

// TODO Time in minutes
// TODO Pause functionality

fn main() {
    let now = Instant::now();
    println!("Starting a timer");

    loop {
        // Clear the terminal window
        print!("\x1B[2J");
        sleep(Duration::new(1, 0));
        println!("{}", now.elapsed().as_secs());
    }
}
