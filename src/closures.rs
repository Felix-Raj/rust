//! # Module Closures
//!
//! Contains example of how to use closures.

#![allow(dead_code)]
pub mod mod_closures {
    use std::thread;
    use std::time::Duration;

    struct Cacher<T>
    where
        T: Fn(u32) -> u32,
    {
        calculation: T,
        value: Option<u32>,
    }

    impl<T> Cacher<T>
    where
        T: Fn(u32) -> u32,
    {
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
            if random_number == 3 {
                println!("Take a break today, remember to stay hyderated!");
            } else {
                println!(
                    "Today, run for {} minutes!",
                    expensive_result.value(intensity)
                );
            }
        }
    }

    fn capture_env() {
        println!("capture env");
        let x = 23;
        // functions cannot do this.
        let equal = |y| y == x;
        let z = 23;
        assert!(equal(z))
    }

    fn closure_that_moves() {
        println!("closure using FnOnce");
        let x = vec![1, 2, 3];
        let e = move |y| y == x;
        // uncomment the following line
        // println!("{:?}", x);
        // try with int, that have Copy Trait implemented, Vec does not have Copy Trait implemented
        let z = vec![1, 2, 3];
        assert!(e(z))
    }

    /// Document for closure main.
    pub fn closures_main() {
        let simulated_user_specific_value = 10;
        let simulated_random_number = 7;

        generate_workout(simulated_user_specific_value, simulated_random_number);
        capture_env();
        closure_that_moves();
        // Closures can capture values from their environment in three ways - which maps to 3 ways fn
        // can take a parameter:
        // 1. taking ownership
        // 2. borrowing mutably
        // 3. borrowing immutably
        // they are
        // 1. FnOnce - takes ownership of variables in env, captures it, move to closure when defined.
        //              can only be called once, as closure cannot take ownership of a variable more
        //              than once
        // 2. FnMut - borrows mutably, so can modify
        // 3. Fn - borrows immutably
        // the equal closure in capture_env implement Fn, x is  not being modified, so x is borrowed
        // immutably
    }
}
