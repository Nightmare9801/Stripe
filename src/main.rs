use std::{io, time::Instant};

use approximater::approximate;

pub mod approximater;

fn main() {
    let now: Instant = Instant::now();
    {
        approximate();
    }
    let elapsed: std::time::Duration = now.elapsed();
    println!("Time Taken: {:.2?}", elapsed);
    println!("Enter any character to exit");
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_n) => {
        }
        Err(error) => println!("error: {error}"),
    }
}
