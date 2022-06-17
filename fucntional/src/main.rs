use std::thread;
use std::time::Duration;

fn main() {
    // closures can capture values from the enclosing scope
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}

fn generate_workout(intensity: u32, random_number: u32) {
    if intensity < 25 {
        println!(
            "Today, do {} pushup",
            simulated_expensive_calculation(intensity)
        );
        println!(
            "Next, do {} situps",
            simulated_expensive_calculation(intensity)
        );
    } else if random_number == 3 {
        println!("Take a break today");
    } else {
        println!(
            "Today run for {} minutes",
            simulated_expensive_calculation(intensity)
        );
    }
}

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("Calculating slowly...");
    thread::sleep(Duration::from_secs(intensity as u64));
    intensity
}

