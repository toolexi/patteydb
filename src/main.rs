use std::time::Instant;

fn main() {
    let mut counter = 0;
    let iterations = 1_000_000;

    let start = Instant::now();

    for _ in 0..iterations {
        counter += 1;
    }

    let duration = start.elapsed();
    println!("Final counter value: {}", counter);
    println!("Time taken: {:?} seconds", duration.as_secs_f64());
}
