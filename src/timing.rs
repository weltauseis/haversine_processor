use std::{
    arch::x86_64::_rdtsc,
    time::{Duration, Instant},
};

fn read_os_timer() -> Instant {
    return Instant::now();
}

pub fn read_cpu_timer() -> u64 {
    unsafe {
        return _rdtsc();
    }
}

pub fn estimate_cpu_frequency(millis_to_wait: u64) -> u64 {
    let cpu_start = read_cpu_timer();

    let os_start: Instant = read_os_timer();
    let mut os_end: Instant = read_os_timer();
    while os_start.elapsed() <= std::time::Duration::from_millis(millis_to_wait) {
        os_end = read_os_timer();
    }

    let cpu_end = read_cpu_timer();

    let os_elapsed = os_end.duration_since(os_start).as_micros();
    let cpu_elapsed = cpu_end - cpu_start;

    let cpu_freq = Duration::from_secs(1).as_micros() * (cpu_elapsed as u128) / os_elapsed;
    return cpu_freq as u64;
}

pub fn elapsed_to_ms(elapsed: u64, freq: u64) -> u64 {
    return (elapsed * 1000) / freq;
}

// main function so that we can compile the file alone for testing
fn main() {
    let args: Vec<String> = std::env::args().collect();

    let millis_to_wait: u64 = if args.len() < 2 {
        1000
    } else {
        args[1].parse().unwrap()
    };

    let freq = estimate_cpu_frequency(millis_to_wait);
    println!("CPU Freq: {} (guessed)", freq);
}
