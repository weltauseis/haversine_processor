use std::{
    arch::x86_64::_rdtsc,
    collections::HashMap,
    time::{Duration, Instant},
};

// RDTSC Utilities

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

// Profiler

pub struct Profiler {
    start: u64,
    end: u64,
    cpu_freq: u64,
    current_section_start: u64,
    sections: HashMap<&'static str, u64>,
}

impl Profiler {
    pub fn new() -> Profiler {
        let sections = HashMap::new();
        let cpu_freq = estimate_cpu_frequency(10);
        let start = read_cpu_timer();

        return Profiler {
            start,
            end: 0,
            cpu_freq,
            current_section_start: 0,
            sections,
        };
    }

    pub fn finalize_and_print_profile(&mut self) {
        self.end = read_cpu_timer();

        let total_elapsed = self.end - self.start;
        println!(
            "\nTotal time : {} ms (CPU freq {})",
            elapsed_to_ms(total_elapsed, self.cpu_freq),
            self.cpu_freq
        );

        for (section_name, time) in &self.sections {
            println!(
                "  {section_name} : {time} ({:.2}%)",
                *time as f64 / total_elapsed as f64 * 100.0
            );
        }
    }

    pub fn start_section(&mut self) {
        self.current_section_start = read_cpu_timer();
    }

    pub fn end_section(&mut self, name: &'static str) {
        let section_end = read_cpu_timer();
        if self.sections.contains_key(&name) {
            let sum = self.sections.get(&name).unwrap();
            self.sections
                .insert(name, sum + (section_end - self.current_section_start));
        } else {
            self.sections
                .insert(name, section_end - self.current_section_start);
        }
    }
}

// main function so that we can compile the file alone for testing
#[allow(dead_code)]
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
