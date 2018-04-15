use std::thread;
use std::time::Duration;

fn main() {
 simulated_expensive_calculation(10);
}

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("Calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}
