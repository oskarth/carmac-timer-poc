use std::thread::sleep;
use std::time::{Duration, Instant};

// TODO Redraw timer, don't append
// TODO Time in minutes
// TODO Pause functionality

fn main() {
    let now = Instant::now();
    println!("Starting a timer");

    loop {
        sleep(Duration::new(1, 0));
        println!("{}", now.elapsed().as_secs());
    }
}
