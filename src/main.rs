use std::sync::Mutex;

use tsc::rdtsc;

fn main() {
    const NUM_ITERATIONS: usize = 1_000_000_000;
    let mut timings = Vec::with_capacity(NUM_ITERATIONS);
    let uncondented_mutex = Mutex::new(());
    for _ in 0..NUM_ITERATIONS {
        let tsc_start = rdtsc();
        let _ = uncondented_mutex.lock().unwrap();
        let tsc_end = rdtsc();
        let diff = tsc_end - tsc_start;
        timings.push(diff);
    }
    let average = (timings.iter().sum::<u64>() as f64) / timings.iter().len() as f64;
    println!("{}", average)
}
