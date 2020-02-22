use std::thread;
use std::time::Duration;


fn generate_workout(intensity: u32, random_number: u32) {
    // Types are inferred. Having explicit type annotation is fine. The compiler infers the type
    // using the first call also, try calling this closure using two different data type, like
    // expensive_closure(23);
    // expensive_closure(String::from("hello"));
    let expensive_closure = |num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };
    if intensity < 25 {
        println!("Today {} pushups!", expensive_closure(intensity));
        println!("Next do {} situps!", expensive_closure(intensity));
    } else {
        if random_number==3 {
            println!("Take a break today, remember to stay hyderated!");
        } else {
            println!("Today, run for {} minutes!", expensive_closure(intensity));
        }
    }
}

fn main() {
    let simulated_user_specific_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specific_value, simulated_random_number);
}