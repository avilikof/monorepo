use std::time::Instant;

fn main() {
    println!("Hello, world!");
    let mut n: u64 = 0;
    let start_time = Instant::now();
    while n < 1_000_000_000 {
        n += 1;
    }
    let end_time = start_time.elapsed();
    let t = end_time.as_nanos();
    println!("Elapsed: {t}nano")
}
