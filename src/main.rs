use std::thread;
use std::time::Duration;

struct Cacher<T> where T: Fn(u32) -> u32 {
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T> where T:Fn(u32) -> u32 {
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    // Types are inferred. Having explicit type annotation is fine. The compiler infers the type
    // using the first call also, try calling this closure using two different data type, like
    // expensive_closure(23);
    // expensive_closure(String::from("hello"));

    // Each closure have it's own anonymous type, even if two closure have same signatures their
    // type are considered to be different.
    // All closure implements one of the traits - Fn, FnMut, FnOnce
    // (https://doc.rust-lang.org/book/ch13-01-closures.html#capturing-the-environment-with-closures)
    // Functions also implement this traits.
    // Closures can capture their environment.
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });
    if intensity < 25 {
        println!("Today {} pushups!", expensive_result.value(intensity));
        println!("Next do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number==3 {
            println!("Take a break today, remember to stay hyderated!");
        } else {
            println!("Today, run for {} minutes!", expensive_result.value(intensity));
        }
    }
}

fn main() {
    let simulated_user_specific_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specific_value, simulated_random_number);
}