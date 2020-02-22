use std::thread;
use std::time::Duration;

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn generate_workout(intensity: u32, random_number: u32) {
    let expensive_result = simulated_expensive_calculation(intensity);
    if intensity < 25 {
        println!("Today {} pushups!", expensive_result);
        println!("Next do {} situps!", expensive_result);
    } else {
        if random_number==3 {
            println!("Take a break today, remember to stay hyderated!");
        } else {
            println!("Today, run for {} minutes!", expensive_result);
        }
    }
}

fn main() {
    let simulated_user_specific_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specific_value, simulated_random_number);
}